#[macro_use]

pub mod helpers;
pub mod map;
pub mod player;
pub mod server_engine;
pub mod client_engine;
pub mod transport;
pub mod msg;
pub mod log;

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
        server_engine::ServerEngine,
        client_engine::{ClientEngine, Builder},
    };
    use std::{sync::{Arc, Mutex, mpsc::sync_channel}};

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
        fn start(mut self) {
            std::thread::spawn(move || {
                for _ in self.ce.state_receiver() {
                    *self.count.lock().unwrap() += 1;
                }
            });
        }
    }

    #[test]
    fn it_works() {
        let count = Arc::new(Mutex::new(0u8));
        let se = ServerEngine::new();
        let mut builder = Builder::new();
        let c_s_send = se.get_sender();
        {
            let (s_c_send, s_c_recv) = sync_channel::<Arc<Envelope>>(0);
            let e = Envelope::Register(s_c_send);
            c_s_send.send((ClientHeader::new(e.msg_type(), 0, 1), e)).unwrap();
            builder.setup_transport(move |s| {
                for e_arc in s_c_recv {
                    s.send((ServerHeader::new(e_arc.msg_type()), (*e_arc).clone())).unwrap();
                }
            }, move |r| {
                for m in r {
                    c_s_send.send(m).unwrap();
                }
            });
        }
        builder.wait_assignment();
        builder.create_user("Smiley".to_owned());
        builder.wait_map();
        let (ce, state) = builder.build();
        let mc = MockClient::new(ce, count.clone());
        mc.start();
        std::thread::sleep(std::time::Duration::from_secs(3));
        assert_eq!(*count.lock().unwrap(), 0);
        assert_eq!(state.user_character.read().unwrap().name, "Smiley".to_owned());
    }
}
