
pub fn debug_buf_to_byte(buf: &[u8]) -> String {
    let mut s = String::new();
    for i in 0..buf.len() {
        s.push_str(&format!("{:x} ", buf[i]))
    }
    s
}