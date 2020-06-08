use std::{
    sync::{Arc, mpsc::Receiver},
    net::TcpStream,
    io::{BufWriter, Write},
};
use emojibomb::{
    transport::{ClientHeader},
    msg::Envelope,
};

pub fn input_to_cengine_loop(
    stream_ptr: Arc<TcpStream>
)-> impl FnOnce(Receiver<(ClientHeader, Envelope)>) + Send + 'static {
    move |r| {
        let mut buf_writer: BufWriter<&TcpStream> = BufWriter::new(&stream_ptr);
        for (h, e) in r {
            h.write_to(&mut buf_writer).expect("");
            e.write_to(&mut buf_writer).expect("");
            buf_writer.flush().expect("");
        }
    }
}