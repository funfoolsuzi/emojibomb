use super::actions::*;
use crate::{
    transport::{ClientHeader},
    state::GameState,
    msg::{Envelope},
    player,
};
use std::{
    sync::{Arc, RwLock, mpsc::{Receiver, SyncSender}}
};
use rand::RngCore;

pub(super) fn handler(
    in_ui_r: Receiver<Action>,
    out_net_s: SyncSender<(ClientHeader, Envelope)>,
    msg_map: Arc<RwLock<std::collections::HashMap<u32, Envelope>>>,
    state: GameState,
) -> impl FnOnce() {
    move || {
        let mut thread_rng = rand::thread_rng();
        for a in in_ui_r {
            handle_action(&a, &out_net_s, &msg_map, &mut thread_rng, &state);
        }
    }
}

fn handle_action(
    a: &Action,
    out_net_s: &SyncSender<(ClientHeader, Envelope)>,
    msg_map: &Arc<RwLock<std::collections::HashMap<u32, Envelope>>>,
    rng: &mut rand::rngs::ThreadRng,
    state: &GameState,
) {
    match a {
        Action::Move(d) => handle_move(d, &out_net_s, &msg_map, rng, &state),
    }
}

fn handle_move(
    d: &Direction,
    out_net_s: &SyncSender<(ClientHeader, Envelope)>,
    msg_map: &Arc<RwLock<std::collections::HashMap<u32, Envelope>>>,
    rng: &mut rand::rngs::ThreadRng,
    state: &GameState,
) {
    let user = state.user_character.read().unwrap();
    let map = state.map.read().unwrap();
    if let Some(new_coord) = move_coord(&d, &user.coord) {
        if map.get(new_coord.0 as usize, new_coord.1 as usize).is_none() {
            return
        }
        // TODO: check if this slot has another player
        let msg_id = rng.next_u32();
        let e = Envelope::PlayerMove(Box::new(player::MoveMsg{id: user.id, coord: new_coord}));
        let h = ClientHeader::new(e.msg_type(), user.id, msg_id);
        msg_map.write().unwrap().insert(msg_id, e.clone());
        out_net_s.send((h, e)).unwrap();
    }
}

fn move_coord(d: &Direction, coord: &(u16, u16)) -> Option<(u16, u16)> {
    match d {
        Direction::Up => Some((coord.0.checked_sub(1)?, coord.1)),
        Direction::Down => Some((coord.0.checked_add(1)?, coord.1)),
        Direction::Left => Some((coord.0, coord.1.checked_sub(1)?)),
        Direction::Right => Some((coord.0, coord.1.checked_add(1)?)),
    }
}