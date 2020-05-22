mod client_msg;
mod server;

use emojibomb::{server_engine::ServerEngine};
use server::Server;

fn main() {
    let se = ServerEngine::new();
    let server = Server::new(se);
    server.start("0.0.0.0:8888").unwrap().join().unwrap().unwrap();
}
