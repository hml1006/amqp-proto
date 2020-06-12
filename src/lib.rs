
pub mod frame;
pub mod basic_types;
pub mod error;

pub use basic_types::{Octet, Short, Long, LongLong, ShortStr, LongStr, AsBytes};

#[cfg(test)]
mod tests {
    use crate::{ShortStr, LongStr, AsBytes};

    #[test]
    fn test_short_str() {
        let short_str = ShortStr::from_bytes(b"hello");
        let ret = [0x5u8, b'h', b'e', b'l', b'l', b'o'];
        assert_eq!(short_str.ok().unwrap().as_bytes(), ret);
    }

    #[test]
    fn test_long_str() {
        let long_str = LongStr::from_bytes(b"hello");
        let ret = [0x0u8, 0x0u8, 0x0u8, 0x5u8, b'h', b'e', b'l', b'l', b'o'];
        assert_eq!(long_str.ok().unwrap().as_bytes(), ret);
    }
}
