mod server2client;
mod input2server;
mod states;
mod ui;

use std::{
    collections::HashMap,
    io::{BufRead, Write},
    net::{TcpStream, SocketAddr},
    sync::{Arc, RwLock, mpsc::{sync_channel}},
};
use emojibomb::{
    client_engine::ClientEngine,
    player,
    msg::{Envelope, MsgType},
    transport::{ClientHeader, ServerHeader},
    state::GameState,
    map::Map,
};
use crate::{
    server2client::server_to_client_loop,
    input2server::input_to_cengine_loop,
    ui::start_ui,
};

fn main() -> std::io::Result<()> {
    let (id, state, stream) = initialize()?;
    let msg_map = Arc::new(RwLock::new(HashMap::new()));
    let engine = ClientEngine::new(state.clone(), msg_map.clone());
    std::thread::Builder::new().name("server2client".to_owned())
        .spawn(server_to_client_loop(stream.clone(), engine.get_sender()))?;
    let (input2server_sender, cengine2output_receiver) = sync_channel::<Envelope>(0);
    std::thread::Builder::new().name("input2server".to_owned())
        .spawn(input_to_cengine_loop(cengine2output_receiver, stream.clone(), msg_map.clone(), id))?;
    start_ui(state.clone(), engine, input2server_sender)
}


fn initialize() -> std::io::Result<(u8, GameState, Arc<TcpStream>)> {
    let mut stream = setup_connection()?;
    let id = read_initial_id(&mut stream)?;
    let user_character = setup_player(&mut stream, id)?;
    let map = wait_for_map(&mut stream)?;
    let state = GameState {map, user_character, characters: Arc::new(RwLock::new(vec!()))};
    Ok((id, state, Arc::new(stream)))
}

fn setup_connection() -> std::io::Result<TcpStream> {
    // let remote_addr = "127.0.0.1:8888".to_owned();
    let remote_addr = from_cli_prompt("Server Address >>")?;
    let server: SocketAddr = remote_addr.parse().unwrap();
    let stream = TcpStream::connect_timeout(&server, std::time::Duration::from_secs(10));
    if stream.is_ok() {
        println!("connection established.");
    }
    stream
}

fn from_cli_prompt(prompt: &'static str) -> std::io::Result<String> {
    print!("{}", prompt);
    std::io::stdout().flush()?;
    let stdin = std::io::stdin();
    let mut input = String::new();
    stdin.lock().read_line(&mut input)?;
    input = input.trim_matches('\n').to_owned();
    Ok(input)
}

fn read_initial_id(stream: &mut TcpStream) -> std::io::Result<u8> {
    let env = Envelope::read_from(stream, MsgType::SlotReserved)?;
    if let Envelope::SlotReserved(m) = env {
        println!("client ID received from server");
        if let Some(client_id) = m.0 {
            return Ok(client_id);
        }
    }
    Err(std::io::Error::new(std::io::ErrorKind::Other, "server responds failed register"))
}

fn setup_player(stream: &mut TcpStream, id: u8) -> std::io::Result<Arc<RwLock<player::Player>>> {
    // let name = "xiwen".to_owned();
    let name = from_cli_prompt("Name >>")?;
    let new_player_msg = player::AttachMsg::new(id, name.as_bytes());
    let header = ClientHeader::new(
        MsgType::PlayerAdd,
        id, 0x5555,
    );
    header.write_to(stream)?;
    new_player_msg.write_to(stream)?;
    stream.flush()?;
    println!("player data sent to server");
    let header = ServerHeader::read_from(stream)?;
    let env = Envelope::read_from(stream, header.mtype())?;
    if let Envelope::PlayerCreated(m) = env {
        println!("player created with id {}", id);
        Ok(Arc::new(RwLock::new((*m).into())))
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::Other, "server should respond PlayerCreatedMsg"))
    }
}

fn wait_for_map(stream: &mut TcpStream) -> std::io::Result<Arc<RwLock<Map>>> {
    println!("waiting for map...");
    let header = ServerHeader::read_from(stream)?;
    let env = Envelope::read_from(stream, header.mtype())?;
    if let Envelope::MapCreate(mmsg) = env {
        println!("map data received from server");
        Ok(Arc::new(RwLock::new((*mmsg).into())))
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid map message"))
    }
}
