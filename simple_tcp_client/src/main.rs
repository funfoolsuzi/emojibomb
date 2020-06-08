mod server2client;
mod input2server;
mod states;
mod ui;

use std::{
    io::{BufRead, Write},
    net::{TcpStream, SocketAddr},
    sync::{Arc},
};
use emojibomb::{
    client_engine::{Builder},
};
use crate::{
    server2client::server_to_client_loop,
    input2server::input_to_cengine_loop,
    ui::start_ui,
};

fn main() -> std::io::Result<()> {
    let mut builder = Builder::new();
    setup_transport(&mut builder)?;
    builder.wait_assignment();
    println!("assigned slot");
    builder.create_user(from_cli_prompt("Name >>")?);
    // builder.create_user("smiley".to_owned());
    println!("user created on server");
    builder.wait_map();
    println!("map data received");
    let (engine, state) = builder.build();
    start_ui(state.clone(), engine)
}

fn setup_transport(builder: &mut Builder) -> std::io::Result<()> {
    let remote_addr = from_cli_prompt("Server Address >>")?;
    // let remote_addr = "127.0.0.1:8888";
    let server: SocketAddr = remote_addr.parse().unwrap();
    let stream = Arc::new(TcpStream::connect_timeout(&server, std::time::Duration::from_secs(10))?);
    builder.setup_transport(server_to_client_loop(stream.clone()), input_to_cengine_loop(stream));
    Ok(())
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
