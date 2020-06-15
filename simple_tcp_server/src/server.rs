use emojibomb::{
    server_engine::ServerEngine,
    transport::{ClientHeader, ServerHeader},
    msg::{Envelope},
    log,
};
use std::{
    sync::{Arc, mpsc::{sync_channel, SyncSender, Receiver}},
    net::{TcpListener, TcpStream, Shutdown},
    io::{Result as IOResult, Write},
    thread::Builder,
};
use crate::client_msg::*;

pub struct Server {
    engine: ServerEngine,
}

impl Server {
    pub fn new(engine: ServerEngine) -> Self {
        Self{ engine }
    }
    pub fn start(&self, addr: &str) -> IOResult<std::thread::JoinHandle<IOResult<()>>> {
        let listener = TcpListener::bind(addr)?;
        log::info!("bound to {}", addr);
        let sender = self.engine.get_sender();
        Builder::new().name("listener_t".to_owned()).spawn(move|| -> IOResult<()>{
            for res in listener.incoming() {
                let stream = res?;
                log::info!("Connection established from {}", stream.peer_addr()?);
                if let Some(_e) = handle_client_connection(
                    stream,
                    sender.clone(),
                ).err() {
                    unimplemented!()
                }
            }
            Ok(())
        })
    }
}

fn handle_client_connection(
    mut stream: TcpStream,
    engine_sender: SyncSender<(ClientHeader, Envelope)>,
) -> IOResult<()> {
    let (tx, rx) = sync_channel::<Arc<Envelope>>(0);
    engine_sender.send((ClientHeader::default(), Envelope::Register(tx))).unwrap();
    let res = rx.recv().unwrap();
    ServerHeader::new(res.msg_type()).write_to(&mut stream)?;
    res.write_to(&mut stream)?;
    stream.flush()?;
    if let Envelope::SlotReserved(m) = *res {
        if let Some(client_id) = m.0 {
            log::info!("sent assigned client id {}", client_id);
            let stream_ptr = Arc::new(stream);
            Builder::new().name(format!("client_{}_read_t", client_id))
            .spawn(from_client_loop(stream_ptr.clone(), engine_sender, client_id))?;
            Builder::new().name(format!("client_{}_write_t", client_id))
            .spawn(from_engine_loop(stream_ptr, rx, client_id))?;
        }
    }
    Ok(())
}

fn from_client_loop(
    stream_ptr: Arc<TcpStream>,
    engine_sender: SyncSender<(ClientHeader, Envelope)>,
    player_id: u8,
) -> impl Fn() -> IOResult<()> {
    move || {
        let client_msg_iter = ClientMsgIterator::new(&stream_ptr);
        let addr = stream_ptr.peer_addr()?;
        for (header, envelope) in client_msg_iter {
            send_engine_with_retry(&engine_sender, (header, envelope), 3, "Error sending msg to engine");
        }
        log::info!("connection closed: {} player_id: {}", addr, player_id);
        engine_sender.send((ClientHeader::default(), Envelope::PlayerDelete(emojibomb::player::DeleteMsg{id: player_id}))).unwrap();
        Ok(())
    }
}

fn from_engine_loop(stream_ptr: Arc<TcpStream>, rx: Receiver<Arc<Envelope>>, id: u8) -> impl Fn() -> IOResult<()> {
    move || {
        let mut tcp_stream_write_buf: std::io::BufWriter<&TcpStream> = std::io::BufWriter::new(&stream_ptr);
        for msg in &rx {
            if let Some(e) = transmit_to_client(&mut tcp_stream_write_buf, msg, id).err() {
                log::error!("error transmitting to client: {}\n shutting down connection", e);
                break
            }
        }
        stream_ptr.shutdown(Shutdown::Both)
    }
}

fn send_engine_with_retry(
    engine_sender: &SyncSender<(ClientHeader, Envelope)>,
    msg: (ClientHeader, Envelope),
    retry: usize,
    fail: &'static str,
) {
    if retry == 0 {
        return
    }
    if let Some(e) = engine_sender.send(msg).err() {
        log::warn!("{} (retry left: {}) {}", fail, retry, e);
        send_engine_with_retry(engine_sender, e.0, retry-1, fail)
    }
}

fn transmit_to_client(
    tcp_stream_write_buf: &mut std::io::BufWriter<&TcpStream>,
    envelope: Arc<Envelope>,
    id: u8,
) -> IOResult<()> {
    let header = ServerHeader::new(envelope.msg_type());
    header.write_to(tcp_stream_write_buf)?;
    envelope.write_to(tcp_stream_write_buf)?;
    log::info!("msg type:{} sending to client#{}", header.mtype() as u16, id);
    tcp_stream_write_buf.flush()
}
