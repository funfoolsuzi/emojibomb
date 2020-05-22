
use std::{
    io::{BufReader},
    sync::Arc,
};
use emojibomb::{
    transport::ClientHeader,
    msg::Envelope,
};

pub struct ClientMsgIterator<'a>{
    br: BufReader<&'a std::net::TcpStream>,
}

impl<'a> ClientMsgIterator<'a> {
    pub fn new(stream: &'a Arc<std::net::TcpStream>) -> Self {
        ClientMsgIterator{
            br: BufReader::new(stream),
        }
    }
}

impl Iterator for ClientMsgIterator<'_> {
    type Item = (ClientHeader, Envelope);
    fn next(&mut self) -> Option<(ClientHeader, Envelope)> {
        match ClientHeader::read_from(&mut self.br).and_then(|header|{
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
