mod client_msg;
mod server;

use emojibomb::{server_engine::ServerEngine, log};
use server::Server;

fn main() {
    log::init_stdout_logging(log::Level::Debug);
    let se = ServerEngine::new();
    let server = Server::new(se);
    server.start("0.0.0.0:8888").unwrap().join().unwrap().unwrap();
}
