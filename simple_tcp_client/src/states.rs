use std::sync::{
    Arc, RwLock, Mutex
};
use emojibomb::{
    map::Map,
    player
};

#[derive(Clone)]
pub struct States {
    pub map: Arc<RwLock<Map>>,
    pub user_character: Arc<Mutex<player::Player>>,
}