
pub fn debug_buf_to_byte(buf: &[u8]) -> String {
    let mut s = String::new();
    for i in 0..buf.len() {
        s.push_str(&format!("{:x} ", buf[i]))
    }
    s
}

macro_rules! ensure_size {
    ($type: path, $size: expr) => {
        assert_eq!(std::mem::size_of::<$type>(), $size);
    };
}