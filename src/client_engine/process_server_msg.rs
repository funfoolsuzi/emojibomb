use crate::{
    transport::{ServerHeader},
    state::GameState,
    msg::{Envelope},
    player,
};
use std::{
    sync::{Arc, RwLock, mpsc::{Receiver, SyncSender}}
};

pub(super) fn handler (
    in_net_r: Receiver<(ServerHeader, Envelope)>,
    out_ui_s: SyncSender<GameState>,
    msg_map: Arc<RwLock<std::collections::HashMap<u32, Envelope>>>,
    state: GameState,
) -> impl FnOnce() {
    move || {
        for (_header, env) in in_net_r {
            match env {
                Envelope::Confirm(m) => 
                    confirm_handler(&m, &msg_map, &state, &out_ui_s),
                Envelope::PlayerCreated(m) =>
                    player_created_handler(*m, &state, &out_ui_s),
                Envelope::PlayerMove(m) =>
                    player_move_handler(&m, &state, &out_ui_s),
                Envelope::PlayerDelete(m) =>
                    player_delete_handler(&m, &state, &out_ui_s),
                _ => {}
            }
        }        
    }
}

fn confirm_handler(
    confirm: &crate::msg::ConfirmMsg,
    msg_map: &Arc<RwLock<std::collections::HashMap<u32, Envelope>>>,
    state: &GameState,
    ui_sender: &SyncSender<GameState>,
) {
    let msg_id = confirm.msg_id();
    if let Some(saved_msg) = msg_map.write().unwrap().remove(&msg_id) {
        if !confirm.valid() { return }
        match saved_msg {
            Envelope::PlayerMove(m) =>
                confirm_player_move_handler(&*m, state, ui_sender),
            _ => {}
        }
    } else {
        println!("msg id({:x}) from server confirmation doesn't exist in msg_list", msg_id);
    }
}

fn confirm_player_move_handler(
    msg: &crate::player::MoveMsg,
    state: &GameState,
    ui_sender: &SyncSender<GameState>,
) {
    state.user_character.write().unwrap().coord = msg.coord;
    ui_sender.send(state.clone()).unwrap();
}

fn player_created_handler(
    m: player::CreatedMsg,
    state: &GameState,
    ui_sender: &SyncSender<GameState>,
) {
    let mut p = state.characters.write().unwrap();
    p.push(m.into());
    ui_sender.send(state.clone()).unwrap();
}

fn player_move_handler(
    m: &player::MoveMsg,
    state: &GameState,
    ui_sender: &SyncSender<GameState>,
) {
    let mut players = state.characters.write().unwrap();
    if let Some(player) = players.iter_mut().find(|p| p.id == m.id) {
        player.coord = m.coord;
        ui_sender.send(state.clone()).unwrap();
    }
}

fn player_delete_handler(
    m: &player::DeleteMsg,
    state: &GameState,
    ui_sender: &SyncSender<GameState>,
) {
    let mut players = state.characters.write().unwrap();
    players.retain(|p| p.id != m.id);
    ui_sender.send(state.clone()).unwrap();
}