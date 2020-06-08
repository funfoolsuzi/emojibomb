use super::actions::Action;
use crate::{
    state::GameState,
};
use std::{
    sync::{mpsc::{Receiver, SyncSender}}
};

pub struct ClientEngine {
    pub(super) in_ui_s: SyncSender<Action>,
    pub(super) out_ui_r: Receiver<GameState>,
}

impl ClientEngine {
    pub fn user_action_sender(&self) -> SyncSender<Action> {
        self.in_ui_s.clone()
    }
    pub fn state_receiver(&mut self) -> &Receiver<GameState> {
        &self.out_ui_r
    }
}

