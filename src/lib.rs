#[macro_use]
pub mod helpers;

pub mod map;
pub mod player;
pub mod server_engine;
pub mod client_engine;
pub mod transport;
pub mod msg;

pub mod state {
    use std::sync::{Arc, RwLock};

    #[derive(Clone)]
    pub struct GameState {
        pub map: Arc<RwLock<crate::map::Map>>,
        pub characters: Arc<RwLock<Vec<crate::player::Player>>>,
        pub user_character: Arc<RwLock<crate::player::Player>>,
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        transport::{ClientHeader, ServerHeader},
        msg::{Envelope},
        state::GameState,
        server_engine::ServerEngine,
        client_engine::ClientEngine,
    };
    use std::{sync::{Arc, Mutex, RwLock, mpsc::{SyncSender, sync_channel}}};

    struct MockServer {
        se: ServerEngine,
        count: Arc<Mutex<u8>>,
    }

    impl MockServer {
        fn new(
            se: ServerEngine,
            count: Arc<Mutex<u8>>,
        ) -> Self {
            Self {se, count}
        }
        fn add_client(&mut self, client_sender: SyncSender<(ServerHeader, Envelope)>) {
            let (tx, rx) = sync_channel::<Arc<Envelope>>(0);
            let se_sender = self.se.get_sender();
            se_sender.send((ClientHeader::default(), Envelope::Register(tx))).unwrap();
            let first_msg = rx.recv().unwrap();
            if let Envelope::SlotReserved(r) = *first_msg {
                if let Some(client_id) = r.0 {
                    println!("client_id:{}", client_id);
                    std::thread::spawn(move|| {
                        for env in rx {
                            client_sender.send((
                                ServerHeader::new(env.msg_type()),
                                (*env).clone()
                            )).unwrap();
                        }
                    });
                } else { panic!("first response from server should contain client id")}
            } else { panic!("first msg should be of type Register")}
        }
    }

    struct MockClient {
        ce: ClientEngine,
        count: Arc<Mutex<u8>>,
    }

    impl MockClient {
        fn new(
            ce: ClientEngine,
            count: Arc<Mutex<u8>>,
        ) -> Self {
            Self{ce, count}
        }
        fn start(self) {
            std::thread::spawn(move || {
                for _ in self.ce {
                    *self.count.lock().unwrap() += 1;
                }
            });
        }
    }

    #[test]
    fn it_works() {
        let count = Arc::new(Mutex::new(0u8));
        let se = ServerEngine::new();
        let ce = ClientEngine::new(
            GameState{
                map: Arc::new(RwLock::new(crate::map::Map::default())),
                characters: Arc::new(RwLock::new(vec!())),
                user_character: Arc::new(RwLock::new(crate::player::Player::new(
                    &[0x1, 0x2], 
                    0,
                    (0, 0),
                    100
                ))),
            },
            Arc::new(RwLock::new(std::collections::HashMap::new())),
        );
        let mut ms = MockServer::new(se, count.clone());
        let mc = MockClient::new(ce, count.clone());
        let ce_sender = mc.ce.get_sender();
        mc.start();
        ms.add_client(ce_sender);
        std::thread::sleep(std::time::Duration::from_secs(3));
        assert_eq!(*count.lock().unwrap(), 0);
    }
}
