use crate::{
    map::{Map},
    player,
    transport::*,
    msg::*,
};
use std::{
    thread::JoinHandle,
    sync::{mpsc::{sync_channel, Receiver, SyncSender}, Arc}
};

struct MsgLoopArgvs {
    in_r: Receiver<(ClientHeader, Envelope)>,
    map: Map,
    players: player::Store,
}

pub struct ServerEngine {
    in_s: SyncSender<(ClientHeader, Envelope)>,
    msg_loop_args: Option<MsgLoopArgvs>,
}

impl ServerEngine {
    pub fn new() -> Self {
        let (in_s, in_r) = sync_channel::<(ClientHeader, Envelope)>(0);
        let mut se = ServerEngine {
            in_s,
            msg_loop_args: Some(MsgLoopArgvs {
                map: Map::default(),
                players: player::Store::default(),
                in_r,
            })
        };
        se.start();
        se
    }
    fn start(&mut self) -> JoinHandle<()> {
        let to_msg_loop = self.msg_loop_args.take().unwrap();
        std::thread::spawn(move || {
            msg_loop(to_msg_loop)
        })
    }
    pub fn get_sender(&self) -> SyncSender<(ClientHeader, Envelope)> {
        self.in_s.clone()
    }
}

fn msg_loop(mut s: MsgLoopArgvs) {
    let mut msg_counter = 0usize;
    for (header, envelope) in s.in_r {
        println!("server engine msg#{} received", msg_counter);
        if !process_msg(envelope, &mut s.map, &mut s.players, header.msg_id()) {
            println!("shutting down");
            break
        }
        msg_counter += 1;
    }
}

fn process_msg(
    envelope: Envelope,
    map: &mut Map,
    players: &mut player::Store,
    msg_id: u32,
) -> bool {
    match envelope {
        Envelope::ShutDown => {
            return false
        },
        Envelope::Register(sender) => {
            sender.send(Arc::new(Envelope::SlotReserved(crate::player::SlotReservedMsg(players.reserve(sender.clone()))))).unwrap();
        },
        Envelope::PlayerDelete(m) => handle_delete_player(players, m),
        Envelope::PlayerAdd(m) => m.act(players, map),
        Envelope::PlayerMove(m) => handle_player_move(players, map, &m, msg_id),
        _ => {},
    };
    true
}

fn handle_player_move(players: &mut player::Store, map: &Map, msg: &player::MoveMsg, msg_id: u32) {
    let success = players.relocate(msg, map.get_size_u16());
    let status = if success {
        players.send_except(Some(msg.id), Arc::new(Envelope::PlayerMove(Box::new(msg.clone()))));
        StatusCode::OK
    } else { StatusCode::InvalidMove };
    players.get_sender(msg.id).unwrap().send(Arc::new(Envelope::Confirm(Box::new(ConfirmMsg::new(msg_id, status))))).unwrap();
}

fn handle_delete_player(players: &mut player::Store, msg: player::DeleteMsg) {
    players.remove(msg.id);
    players.send_except(None, Arc::new(Envelope::PlayerDelete(msg)));
}