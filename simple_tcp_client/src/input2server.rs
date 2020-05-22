use std::{
    sync::{Arc, RwLock, mpsc::Receiver},
    net::TcpStream,
    io::{BufWriter, Write},
    collections::HashMap,
};
use emojibomb::{
    transport::{ClientHeader},
    msg::Envelope,
};

pub fn input_to_cengine_loop(
    rx: Receiver<Envelope>,
    stream_ptr: Arc<TcpStream>,
    msg_list: Arc<RwLock<HashMap<u32, Envelope>>>,
    id: u8,
)-> impl Fn() -> std::io::Result<()> {
    use rand::RngCore;
    move || {
        let mut buf_writer: BufWriter<&TcpStream> = BufWriter::new(&stream_ptr);
        let mut thread_rng = rand::thread_rng();
        for env in rx.iter() {
            let msg_id = thread_rng.next_u32();
            let header = ClientHeader::new(env.msg_type(), id, msg_id);
            msg_list.write().unwrap().insert(msg_id, env.clone());
            header.write_to(&mut buf_writer)?;
            env.write_to(&mut buf_writer)?;
            buf_writer.flush()?;
        }
        Ok(())
    }
}