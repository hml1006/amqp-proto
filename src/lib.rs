
pub mod frame;
pub mod basic_types;
pub mod error;

pub use basic_types::{Timestamp, ShortStr, LongStr, WriteToBuf, Decimal, FieldName, FieldValue, FieldArray, FieldTable, BytesArray};

#[cfg(test)]
mod tests {
    use crate::{WriteToBuf, ShortStr, LongStr, Decimal, FieldValue, FieldTable};
    use bytes::{BytesMut, BufMut};
    use std::borrow::BorrowMut;
    use crate::error::{Error, ErrorKind};
    use crate::basic_types::{FieldName, FieldArray};

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

        let value = FieldValue::from_i16(-0x1234i16);
        let mut ret = BytesMut::with_capacity(8);
        ret.put_u8(b's');
        ret.put_i16(-0x1234i16);
        buf.clear();
        value.write_to_buf(buf.borrow_mut());
        assert_eq!(&buf[..], &ret[..]);

        let value = FieldValue::from_u16(0x1234u16);
        let mut ret = BytesMut::with_capacity(8);
        ret.put_u8(b'u');
        ret.put_u16(0x1234u16);
        buf.clear();
        value.write_to_buf(buf.borrow_mut());
        assert_eq!(&buf[..], &ret[..]);

        let value = FieldValue::from_i32(-0x1i32);
        let mut ret = BytesMut::with_capacity(32);
        ret.put_u8(b'I');
        ret.put_i32(-0x1i32);
        buf.clear();
        value.write_to_buf(buf.borrow_mut());
        assert_eq!(&buf[..], &ret[..]);

        let value = FieldValue::from_u32(0x12345678u32);
        let mut ret = BytesMut::with_capacity(8);
        ret.put_u8(b'i');
        ret.put_u32(0x12345678u32);
        buf.clear();
        value.write_to_buf(buf.borrow_mut());
        assert_eq!(&buf[..], &ret[..]);

        let value = FieldValue::from_i64(-0x12345678i64);
        let mut ret = BytesMut::with_capacity(64);
        ret.put_u8(b'l');
        ret.put_i64(-0x12345678i64);
        buf.clear();
        value.write_to_buf(buf.borrow_mut());
        assert_eq!(&buf[..], &ret[..]);

        let value = FieldValue::from_u64(0x12345678u64);
        let mut ret = BytesMut::with_capacity(8);
        ret.put_u8(b'L');
        ret.put_u64(0x12345678u64);
        buf.clear();
        value.write_to_buf(buf.borrow_mut());
        assert_eq!(&buf[..], &ret[..]);

        let value = FieldValue::from_f32(12345678.12f32);
        let mut ret = BytesMut::with_capacity(64);
        ret.put_u8(b'f');
        ret.put_f32(12345678.12f32);
        buf.clear();
        value.write_to_buf(buf.borrow_mut());
        assert_eq!(&buf[..], &ret[..]);

        let value = FieldValue::from_f64(12345678.12f64);
        let mut ret = BytesMut::with_capacity(64);
        ret.put_u8(b'd');
        ret.put_f64(12345678.12f64);
        buf.clear();
        value.write_to_buf(buf.borrow_mut());
        assert_eq!(&buf[..], &ret[..]);

        let value = FieldValue::from_timestamp(0x12345678u64);
        let mut ret = BytesMut::with_capacity(8);
        ret.put_u8(b'T');
        ret.put_u64(0x12345678u64);
        buf.clear();
        value.write_to_buf(buf.borrow_mut());
        assert_eq!(&buf[..], &ret[..]);

        let value = FieldValue::from_decimal(Decimal::new(2, 0x12345678u32));
        let mut ret = BytesMut::with_capacity(8);
        ret.put_u8(b'D');
        ret.put_u8(2u8);
        ret.put_u32(0x12345678u32);
        buf.clear();
        value.write_to_buf(buf.borrow_mut());
        assert_eq!(&buf[..], &ret[..]);

        let value = FieldValue::from_long_string(LongStr::with_bytes(b"hello").unwrap());
        let mut ret = BytesMut::with_capacity(8);
        ret.put_u8(b'S');
        ret.put_u32(0x5u32);
        ret.put_slice(b"hello");
        buf.clear();
        value.write_to_buf(buf.borrow_mut());
        assert_eq!(&buf[..], &ret[..]);

        let mut array = FieldArray::new();
        let v1 = FieldValue::from_u32(0x12345678u32);
        let v2 = FieldValue::from_long_string(LongStr::with_bytes(b"hello").unwrap());
        array.push(v1);
        array.push(v2);
        let value = FieldValue::from_field_array(array);
        let mut ret = BytesMut::with_capacity(128);
        ret.put_u8(b'A');
        ret.put_u32(15u32);
        ret.put_u8(b'i');
        ret.put_u32(0x12345678u32);
        ret.put_u8(b'S');
        ret.put_u32(0x5u32);
        ret.put_slice(b"hello");
        buf.clear();
        value.write_to_buf(buf.borrow_mut());
        assert_eq!(&buf[..], &ret[..]);

        let mut table = FieldTable::new();
        table.insert(FieldName::with_bytes(b"hello").unwrap(), FieldValue::from_u32(0x12345678u32));
        table.insert(FieldName::with_bytes(b"world").unwrap(), FieldValue::from_long_string(LongStr::with_bytes(b"hello").unwrap()));
        let mut ret = BytesMut::with_capacity(128);
        ret.put_u8(b'F');
        ret.put_u32(27u32);
        for (k, v) in &table {
            if *k == FieldName::with_bytes(b"hello").unwrap() {
                ret.put_u8(5u8);
                ret.put_slice(b"hello");
                ret.put_u8(b'i');
                ret.put_u32(0x12345678u32);
            } else {
                ret.put_u8(5u8);
                ret.put_slice(b"world");
                ret.put_u8(b'S');
                ret.put_u32(5u32);
                ret.put_slice(b"hello");
            }
        }
        let value = FieldValue::from_field_table(table);
        buf.clear();
        value.write_to_buf(buf.borrow_mut());
        assert_eq!(&buf[..], &ret[..]);

        let value = FieldValue::from_bytes_array(LongStr::with_bytes(b"hello").unwrap());
        let mut ret = BytesMut::with_capacity(8);
        ret.put_u8(b'x');
        ret.put_u32(0x5u32);
        ret.put_slice(b"hello");
        buf.clear();
        value.write_to_buf(buf.borrow_mut());
        assert_eq!(&buf[..], &ret[..]);

        let value = FieldValue::from_void();
        let ret = [b'V'];
        buf.clear();
        value.write_to_buf(buf.borrow_mut());
        assert_eq!(&buf[..], ret)
    }
}
