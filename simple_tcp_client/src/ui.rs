use std::sync::{mpsc::SyncSender};
use emojibomb::{
    state::GameState,
    map::{LandScape, Map},
    client_engine::ClientEngine,
    msg::Envelope,
};
use cursive::views::{TextView, LayerPosition};
use cursive::event::{Key};


pub fn start_ui(
    state: GameState,
    engine: ClientEngine,
    server_sender: SyncSender<Envelope>,
) -> std::io::Result<()> {
    let mut siv = cursive::default();
    attach_input_cbs(&mut siv, state.clone(), server_sender);
    let tv = TextView::new(state_to_text(&state));
    std::thread::Builder::new().name("ui_t".to_owned()).spawn(ui_loop(engine, siv.cb_sink().clone()))?;
    siv.add_layer(tv);
    siv.run();
    Ok(())
}

fn attach_input_cbs(siv: &mut cursive::Cursive, state: GameState, sender: SyncSender<Envelope>) {
    siv.add_global_callback(Key::Esc, |s| s.quit());
    siv.add_global_callback('w', move_up(state.clone(), sender.clone()));
    siv.add_global_callback('s', move_down(state.clone(), sender.clone()));
    siv.add_global_callback('a', move_left(state.clone(), sender.clone()));
    siv.add_global_callback('d', move_right(state.clone(), sender.clone()));
}

fn ui_loop(engine: ClientEngine, cbsink: cursive::CbSink) -> impl FnOnce() {
    move || {
        for state in engine {
            cbsink.send(Box::new(move |s| {
                let tv: &mut TextView =  s.screen_mut().get_mut(LayerPosition::FromFront(0)).unwrap().as_any_mut().downcast_mut::<TextView>().unwrap();
                tv.set_content(state_to_text(&state));
                s.refresh();
            })).unwrap();
        }
    }
}

fn state_to_text(state: &GameState) -> String {
    let mut s = String::new();
    let m = state.map.read().unwrap();
    for (r, c, tile) in m.get_iter() {
        if r > 0 && c == 0 {
            s.push('\n');
        }
        let c = match tile.landscape() {
            LandScape::Grass => " x",
            LandScape::Water => " x",
            LandScape::Woods => " x",
        };
        s.push_str(c);
    }
    let mut list = vec![(flatten_coord(&m, state.user_character.read().unwrap().coord, 2), '😃')];
    for p in state.characters.read().unwrap().iter() {
        list.push((flatten_coord(&m, p.coord, 2), '😈'));
    }
    list.sort_by(|i, j| {
        if i.0 > j.0 { std::cmp::Ordering::Less }
        else { std::cmp::Ordering::Greater }
    });
    for i in list {
        s.remove(i.0);
        s.remove(i.0);
        s.insert(i.0, i.1);        
    }
    s
}

fn flatten_coord(map: &Map, coord: (u16, u16), slot_width: usize) -> usize {
    let size = map.get_size();
    (size.1 * slot_width + 1) * coord.0 as usize + (coord.1 as usize * slot_width)
}

fn move_up(state: GameState, sender: SyncSender<Envelope>) -> impl FnMut(& mut cursive::Cursive) {
    move|_| {
        let chara = state.user_character.read().unwrap();
        let mut coord = chara.coord;
        if coord.0 > 0 {
            coord.0 -= 1;
            sender.send(Envelope::PlayerMove(Box::new(emojibomb::player::MoveMsg{id: chara.id, coord}))).unwrap();
        }
    }
}

fn move_down(state: GameState, sender: SyncSender<Envelope>) -> impl FnMut(& mut cursive::Cursive) {
    move|_| {
        let chara = state.user_character.read().unwrap();
        let map = state.map.read().unwrap();
        let mut coord = chara.coord;
        coord.0 += 1;
        if map.get(coord.0 as usize, coord.1 as usize).is_some() {
            sender.send(Envelope::PlayerMove(Box::new(emojibomb::player::MoveMsg{id: chara.id, coord}))).unwrap();
        }
    }
}

fn move_left(state: GameState, sender: SyncSender<Envelope>) -> impl FnMut(& mut cursive::Cursive) {
    move|_| {
        let chara = state.user_character.read().unwrap();
        let mut coord = chara.coord;
        if coord.1 > 0 {
            coord.1 -= 1;
            sender.send(Envelope::PlayerMove(Box::new(emojibomb::player::MoveMsg{id: chara.id, coord}))).unwrap();
        }
    }
}

fn move_right(state: GameState, sender: SyncSender<Envelope>) -> impl FnMut(& mut cursive::Cursive) {
    move|_| {
        let chara = state.user_character.read().unwrap();
        let map = state.map.read().unwrap();
        let mut coord = chara.coord;
        coord.1 += 1;
        if map.get(coord.0 as usize, coord.1 as usize).is_some() {
            sender.send(Envelope::PlayerMove(Box::new(emojibomb::player::MoveMsg{id: chara.id, coord}))).unwrap();
        }
    }
}