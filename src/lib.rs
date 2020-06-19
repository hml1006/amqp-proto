
pub mod frame;
pub mod basic_types;
pub mod error;

pub use basic_types::{Timestamp, ShortStr, LongStr, WriteToBuf, Decimal, FieldName, FieldValue};

#[cfg(test)]
mod tests {
    use crate::{WriteToBuf, ShortStr, LongStr, Decimal, FieldValue};
    use bytes::{BytesMut, BufMut};
    use std::borrow::BorrowMut;
    use crate::error::{Error, ErrorKind};
    use crate::basic_types::FieldName;

    #[test]
    fn test_short_str() {
        let short_str = ShortStr::with_bytes(b"hello");
        let ret = [0x5u8, b'h', b'e', b'l', b'l', b'o'];
        let mut buf = BytesMut::with_capacity(32);
        short_str.unwrap().write_to_buf(buf.borrow_mut());
        assert_eq!(&buf[..], ret);

        let mut tmp = String::new();
        for _ in 0..=26 {
            tmp.push_str("aaaaabbbbb");
        }

        let short_str = ShortStr::with_bytes(tmp.as_bytes());
        let ret = match short_str.err().unwrap().kind() {
            ErrorKind::StrTooLong => true,
            _ => false
        };
        assert!(ret);
    }

    #[test]
    fn test_long_str() {
        let long_str = LongStr::with_bytes(b"hello");
        let ret = [0x0u8, 0x0u8, 0x0u8, 0x5u8, b'h', b'e', b'l', b'l', b'o'];

        let mut buf = BytesMut::with_capacity(32);
        long_str.unwrap().write_to_buf(buf.borrow_mut());
        assert_eq!(&buf[..], ret);

        let mut tmp = String::new();
        for _ in 0..=8192 {
            tmp.push_str("aaaabbbb");
        }
        let long_str = LongStr::with_bytes(tmp.as_bytes());
        let ret = match long_str.err().unwrap().kind() {
            ErrorKind::StrTooLong => true,
            _ => false
        };
        assert!(ret);
    }

    #[test]
    fn test_decimal() {
        let decimal = Decimal::new(1u8, 0x12345678u32);
        let ret = [1u8, 0x12u8, 0x34u8, 0x56u8, 0x78u8];

        let mut buf = BytesMut::with_capacity(32);
        decimal.write_to_buf(buf.borrow_mut());
        assert_eq!(&buf[..], ret);
    }

    #[test]
    fn test_field_name() {
        let field_name = FieldName::with_bytes(b"hello");
        let mut buf = BytesMut::with_capacity(32);

        field_name.unwrap().write_to_buf(buf.borrow_mut());
        let ret = [0x5u8, b'h', b'e', b'l', b'l', b'o'];
        assert_eq!(&buf[..], ret);

        let mut tmp = String::new();
        for _ in 0..=13 {
            tmp.push_str("aaaaabbbbb");
        }
        let field_name = FieldName::with_bytes(tmp.as_bytes());
        let ret = match field_name.err().unwrap().kind() {
            ErrorKind::StrTooLong => true,
            _ => false
        };
        assert!(ret);

        let field_name = FieldName::with_bytes(b"1ello");
        let ret = match field_name.err().unwrap().kind() {
            ErrorKind::WrongShortStrFirstLetter => true,
            _ => false
        };
        assert!(ret);
    }

    #[test]
    fn test_field_value() {
        let mut buf = BytesMut::with_capacity(8);
        let value = FieldValue::from_bool(true);
        let ret = [b't', 0x1u8];
        value.write_to_buf(buf.borrow_mut());
        assert_eq!(&buf[..], ret);

        let value = FieldValue::from_bool(false);
        let ret = [b't', 0x0u8];
        buf.clear();
        value.write_to_buf(buf.borrow_mut());
        assert_eq!(&buf[..], ret);

        let value = FieldValue::from_i8(-0x1i8);
        let ret: [u8; 2] = [b'b' as u8, -0x1i8 as u8];
        buf.clear();
        value.write_to_buf(buf.borrow_mut());
        assert_eq!(&buf[..], ret);

        let value = FieldValue::from_u8(0x1u8);
        let ret = [b'B', 0x1u8];
        buf.clear();
        value.write_to_buf(buf.borrow_mut());
        assert_eq!(&buf[..], ret);

        let value = FieldValue::from_i32(-0x1i32);
        let mut ret = BytesMut::with_capacity(32);
        ret.put_u8(b'I');
        ret.put_i32(-0x1i32);
        buf.clear();
        value.write_to_buf(buf.borrow_mut());
        assert_eq!(&buf[..], &ret[..]);
    }
}
