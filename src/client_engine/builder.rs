
use super::{ClientEngine, actions::Action};

use crate::{
    transport::{ServerHeader, ClientHeader},
    state::GameState,
    msg::{Envelope, MsgType},
    player,
};
use std::{
    sync::{Arc, RwLock, mpsc::{sync_channel, Receiver, SyncSender}}
};

#[derive(PartialEq)]
enum Stage {
    Created,
    Connected,
    Assigned,
    PlayerCreated,
    Ready
}

pub struct Builder {
    stage: Stage,
    in_net_s: SyncSender<(ServerHeader, Envelope)>,
    in_net_r: Receiver<(ServerHeader, Envelope)>,
    out_ui_s: SyncSender<GameState>,
    in_ui_r: Receiver<Action>,
    out_net_s: SyncSender<(ClientHeader, Envelope)>,
    out_net_r: Option<Receiver<(ClientHeader, Envelope)>>,
    id: Option<u8>,
    player: Option<player::Player>,
    state: Option<GameState>,
    msg_map: Arc<RwLock<std::collections::HashMap<u32, Envelope>>>,
    engine: ClientEngine,
}

impl Builder {
    pub fn new() -> Self {
        let (in_net_s, in_net_r) = sync_channel::<(ServerHeader, Envelope)>(0);
        let (out_net_s, out_net_r) = sync_channel::<(ClientHeader, Envelope)>(0);
        let (in_ui_s, in_ui_r) = sync_channel::<Action>(64);
        let (out_ui_s, out_ui_r) = sync_channel::<GameState>(64);
        Self {
            stage: Stage::Created,
            in_net_r,
            in_net_s,
            out_ui_s,
            out_net_s,
            out_net_r: Some(out_net_r),
            in_ui_r,
            id: None,
            player: None,
            state: None,
            msg_map: Arc::new(RwLock::new(std::collections::HashMap::new())),
            engine: ClientEngine {
                in_ui_s, out_ui_r,
            }
        }
    }
    pub fn setup_transport(
        &mut self,
        in_handler: impl FnOnce(SyncSender<(ServerHeader, Envelope)>) + Send + 'static,
        out_handler: impl FnOnce(Receiver<(ClientHeader, Envelope)>) + Send + 'static,
    ) {
        if self.stage != Stage::Created {
            panic!("setup_transport can only be called at Stage::Created")
        }
        let s = self.in_net_s.clone();
        let r = self.out_net_r.take().expect("");
        std::thread::spawn(move || {
            in_handler(s)
        });
        std::thread::spawn(move || {
            out_handler(r)
        });
        self.stage = Stage::Connected;
    }
    pub fn wait_assignment(&mut self) -> u8 {
        if self.stage != Stage::Connected {
            panic!("wait_assignment can only be called at Stage::Connected")
        }
        let (_, e) = self.in_net_r.recv().expect("receiving slot assignment");
        if let Envelope::SlotReserved(m) = e {
            if let Some(id) = m.0 {
                self.stage = Stage::Assigned;
                self.id = Some(id);
                return id
            }
        }
        panic!("fail to reserve player slot")
    }
    pub fn create_user(&mut self, name: String) {
        if self.stage != Stage::Assigned {
            panic!("create_user can only be called at Stage::Assigned")
        }
        let id = self.id.expect("id shouldn't be none at Stage::Assgined");
        let msg = player::AttachMsg::new(id, name.as_bytes());
        self.out_net_s.send((
            ClientHeader::new(MsgType::PlayerAdd, id, 0x5555),
            Envelope::PlayerAdd(Box::new(msg)),
        )).expect("PlayerAdd msg should be sent");
        let (_, e) = self.in_net_r.recv().expect("a msg should be replied back after sending PlayerAdd msg");
        if let Envelope::PlayerCreated(m) = e {
            self.player = Some((*m).into());
            self.stage = Stage::PlayerCreated;
        } else {
            panic!("a PlayerCreated envelope should be received after sending PlayerAdd")
        }
    }
    pub fn wait_map(&mut self) {
        if self.stage != Stage::PlayerCreated {
            panic!("wait_map can only be called at Stage::PlayerCreated")
        }
        let (_, e) = self.in_net_r.recv().expect("waiting for map data");
        if let Envelope::MapCreate(m) = e {
            let state = GameState {
                characters: Arc::new(RwLock::new(vec!())),
                map: Arc::new(RwLock::new((*m).into())),
                user_character: Arc::new(RwLock::new(self.player.take().expect("player shouldn't be None at Stage::PlayerCreated")))
            };
            self.state = Some(state);
            self.stage = Stage::Ready;
        } else {
            panic!("map data expected")
        }
    }
    pub fn build(mut self) -> (ClientEngine, GameState) {
        if self.stage != Stage::Ready {
            panic!("build can only be called at Stage::Ready")
        }
        let state = self.state.take().unwrap();
        std::thread::spawn(super::process_server_msg::handler(self.in_net_r, self.out_ui_s, self.msg_map.clone(), state.clone()));
        std::thread::spawn(super::process_user_action::handler(self.in_ui_r, self.out_net_s, self.msg_map, state.clone()));
        (self.engine, state)
    }
}