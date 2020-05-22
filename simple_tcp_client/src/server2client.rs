use std::{
    io::{BufReader},
    net::TcpStream,
    sync::{Arc, mpsc::SyncSender},
};

use emojibomb::{
    transport::{ServerHeader},
    msg::Envelope,
};

pub fn server_to_client_loop(
    stream_ptr: Arc<TcpStream>,
    sender: SyncSender<(ServerHeader, Envelope)>,
) -> impl Fn() {
    move || {
        for msg_pair in ServerStreamBufReader::new(&stream_ptr) {
            sender.send(msg_pair).unwrap();
        }
    }
}

struct ServerStreamBufReader<'a> {
    br: BufReader<&'a TcpStream>,
}

impl<'a> ServerStreamBufReader<'a> {
    fn new(stream: &'a Arc<TcpStream>) -> Self {
        Self {
            br: BufReader::new(stream),
        }
    }
}

impl Iterator for ServerStreamBufReader<'_> {
    type Item = (ServerHeader, Envelope);
    fn next(&mut self) -> Option<(ServerHeader, Envelope)> {
        match ServerHeader::read_from(&mut self.br).and_then(|header| {
            Envelope::read_from(&mut self.br, header.mtype()).map(|env| {
                (header, env)
            })
        }) {
            Err(e) => {
                println!("Error receiving msg: {}", e);
                None
            }
            Ok(r) => Some(r)
        }
    }
}
