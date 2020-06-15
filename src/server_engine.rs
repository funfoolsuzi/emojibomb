use crate::{
    map, player,
    transport::*,
    msg::*,
    log,
};
use std::{
    thread::JoinHandle,
    sync::{mpsc::{sync_channel, Receiver, SyncSender}, Arc}
};

struct MsgLoopArgvs {
    in_r: Receiver<(ClientHeader, Envelope)>,
    map: map::Map,
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
                map: map::Map::default(),
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
        log::info!("server engine msg#{} received", msg_counter);
        if !process_msg(envelope, &mut s.map, &mut s.players, header.msg_id()) {
            log::info!("shutting down");
            break
        }
        msg_counter += 1;
    }
}

fn process_msg(
    envelope: Envelope,
    map: &mut map::Map,
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
        Envelope::PlayerDelete(m) => handle_delete_player(m, players),
        Envelope::PlayerAdd(m) => handle_attach_player(*m, players, map),
        Envelope::PlayerMove(m) => handle_player_move(&m, players, map, msg_id),
        _ => {},
    };
    true
}

fn handle_player_move(msg: &player::MoveMsg, players: &mut player::Store, map: &map::Map, msg_id: u32) {
    let success = players.relocate(msg, map.get_size_u16());
    let status = if success {
        players.send_except(Some(msg.id), Arc::new(Envelope::PlayerMove(Box::new(msg.clone()))));
        StatusCode::OK
    } else { StatusCode::InvalidMove };
    players.get_sender(msg.id).unwrap().send(Arc::new(Envelope::Confirm(Box::new(ConfirmMsg::new(msg_id, status))))).unwrap();
}

fn handle_delete_player(msg: player::DeleteMsg, players: &mut player::Store) {
    players.remove(msg.id);
    players.send_except(None, Arc::new(Envelope::PlayerDelete(msg)));
}

fn handle_attach_player(msg: player::AttachMsg, players: &mut player::Store, map: &map::Map) {
    let mut new_player = player::Player::new(
        &msg.name, msg.client_id,
        (0, 0), 100
    );
    let map_size = map.get_size();
    players.attach_player(&mut new_player, (map_size.0 as u16, map_size.1 as u16));
    let env = Envelope::PlayerCreated(Box::new(new_player.into()));
    let ptr = Arc::new(env);
    let mut existing_player_msgs: Vec<Arc<Envelope>> = vec!();
    for entry in players.iter() {
        if let Some(player) = &entry.player {
            if player.id != msg.client_id {
                existing_player_msgs.push(Arc::new(Envelope::PlayerCreated(Box::new(player.clone().into()))));
            }
            entry.sender.send(ptr.clone()).unwrap();                
        }
    }
    let to_new_client = players.get_sender(msg.client_id).unwrap();
    to_new_client.send(
        Arc::new(Envelope::MapCreate(Box::new(map::CreateMsg::new(map.clone()))))
    ).unwrap();
    for m in existing_player_msgs {
        to_new_client.send(m).unwrap();
    }
}