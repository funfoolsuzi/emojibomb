use std::sync::{mpsc::SyncSender};
use emojibomb::{
    state::GameState,
    map::{LandScape, Map},
    client_engine::{ClientEngine, actions::*},
};
use cursive::views::{TextView, LayerPosition};
use cursive::event::{Key};


pub fn start_ui(
    state: GameState,
    engine: ClientEngine,
) -> std::io::Result<()> {
    let mut siv = cursive::default();
    attach_input_cbs(&mut siv, engine.user_action_sender());
    let tv = TextView::new(state_to_text(&state));
    std::thread::Builder::new().name("ui_t".to_owned()).spawn(ui_loop(engine, siv.cb_sink().clone()))?;
    siv.add_layer(tv);
    siv.run();
    Ok(())
}

fn attach_input_cbs(siv: &mut cursive::Cursive, sender: SyncSender<Action>) {
    siv.add_global_callback(Key::Esc, |s| s.quit());
    siv.add_global_callback('w', send_move(sender.clone(), Direction::Up));
    siv.add_global_callback('s', send_move(sender.clone(), Direction::Down));
    siv.add_global_callback('a', send_move(sender.clone(), Direction::Left));
    siv.add_global_callback('d', send_move(sender.clone(), Direction::Right));
}

fn ui_loop(mut engine: ClientEngine, cbsink: cursive::CbSink) -> impl FnOnce() {
    move || {
        for state in engine.state_receiver() {
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

fn send_move(sender: SyncSender<Action>, direction: Direction) -> impl FnMut(&mut cursive::Cursive) {
    move |_| {
        sender.send(Action::Move(direction)).unwrap();
    }
}
