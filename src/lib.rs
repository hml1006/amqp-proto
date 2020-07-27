#![feature(in_band_lifetimes)]

mod class;
mod method;
mod frame;
mod error;

/// Complex amqp types
pub use frame::base::{Timestamp, ShortStr, LongStr, Decimal, FieldName, FieldValue, FieldArray, FieldTable, BytesArray};

/// Method type and id definitions
pub use method::{AccessMethod, BasicMethod, ChannelMethod, ConnectionMethod, ConfirmMethod, ExchangeMethod, QueueMethod, TxMethod, Method, MethodId};

/// Class type and id definitions
pub use class::Class;

/// Content Header frame properties
pub mod properties {
    pub use crate::frame::header::access;
    pub use crate::frame::header::basic;
    pub use crate::frame::header::channel;
    pub use crate::frame::header::connection;
    pub use crate::frame::header::confirm;
    pub use crate::frame::header::exchange;
    pub use crate::frame::header::queue;
    pub use crate::frame::header::tx;
}

/// Method frame arguments
pub mod arguments {
    pub use crate::frame::method::access;
    pub use crate::frame::method::basic;
    pub use crate::frame::method::channel;
    pub use crate::frame::method::connection;
    pub use crate::frame::method::confirm;
    pub use crate::frame::method::exchange;
    pub use crate::frame::method::queue;
    pub use crate::frame::method::tx;
}
pub mod codec {
    pub use crate::frame::frame_codec::{DecodedFrame, FrameCodec};
    pub use crate::frame::base::{ContentHeaderPayload, HeartbeatPayload, MethodPayload, Payload, Frame, ProtocolHeader, Decode, Encode};
}
pub mod err {
    pub use crate::error::FrameDecodeErr;
    pub use crate::error::amqp::{AmqpError, AmqpErrorKind};
}

#[cfg(test)]
mod tests {
    use crate::{ShortStr, LongStr, Decimal, FieldValue, FieldTable, FieldName, FieldArray};
    use crate::frame::method::connection::ConnectionStart;
    use crate::codec::Encode;
    use bytes::{BytesMut, BufMut};
    use std::borrow::BorrowMut;

    #[test]
    fn test_connection_start() {
        let mut connection_start = ConnectionStart::default();
        connection_start.set_version_major(0);
        connection_start.set_version_minor(9);

        assert_eq!(connection_start.version_major(), 0);
    }

    #[test]
    fn test_short_str() {
        let short_str = ShortStr::with_bytes(b"hello");
        let ret = [0x5u8, b'h', b'e', b'l', b'l', b'o'];
        let mut buf = BytesMut::with_capacity(32);
        short_str.unwrap().encode(buf.borrow_mut());
        assert_eq!(&buf[..], ret);

        let mut tmp = String::new();
        for _ in 0..=26 {
            tmp.push_str("aaaaabbbbb");
        }

        let short_str = ShortStr::with_bytes(tmp.as_bytes());
        let ret = match short_str {
            Err(e) => {
                println!("{}", e);
                true
            },
            _ => false
        };
        assert!(ret);
    }

    #[test]
    fn test_long_str() {
        let long_str = LongStr::with_bytes(b"hello");
        let ret = [0x0u8, 0x0u8, 0x0u8, 0x5u8, b'h', b'e', b'l', b'l', b'o'];

        let mut buf = BytesMut::with_capacity(32);
        long_str.unwrap().encode(buf.borrow_mut());
        assert_eq!(&buf[..], ret);

        let mut tmp = String::new();
        for _ in 0..=8192 {
            tmp.push_str("aaaabbbb");
        }
        let long_str = LongStr::with_bytes(tmp.as_bytes());
        let ret = match long_str {
            Err(e) => {
                println!("{}", e);
                true
            },
            _ => false
        };
        assert!(ret);
    }

    #[test]
    fn test_decimal() {
        let decimal = Decimal::new(1u8, 0x12345678u32);
        let ret = [1u8, 0x12u8, 0x34u8, 0x56u8, 0x78u8];

        let mut buf = BytesMut::with_capacity(32);
        decimal.encode(buf.borrow_mut());
        assert_eq!(&buf[..], ret);
    }

    #[test]
    fn test_field_name() {
        let field_name = FieldName::with_bytes(b"hello");
        let mut buf = BytesMut::with_capacity(32);

        field_name.unwrap().encode(buf.borrow_mut());
        let ret = [0x5u8, b'h', b'e', b'l', b'l', b'o'];
        assert_eq!(&buf[..], ret);

        let mut tmp = String::new();
        for _ in 0..=13 {
            tmp.push_str("aaaaabbbbb");
        }
        let field_name = FieldName::with_bytes(tmp.as_bytes());
        let ret = match field_name {
            Err(e) => {
                println!("{}", e);
                true
            },
            _ => false
        };
        assert!(ret);

        let field_name = FieldName::with_bytes(b"1ello");
        let ret = match field_name {
            Err(e) => {
                println!("{}", e);
                true
            },
            _ => false
        };
        assert!(ret);
    }

    #[test]
    fn test_field_value() {
        let mut buf = BytesMut::with_capacity(8);
        let value = FieldValue::from_bool(true);
        let ret = [b't', 0x1u8];
        value.encode(buf.borrow_mut());
        assert_eq!(&buf[..], ret);

        let value = FieldValue::from_bool(false);
        let ret = [b't', 0x0u8];
        buf.clear();
        value.encode(buf.borrow_mut());
        assert_eq!(&buf[..], ret);

        let value = FieldValue::from_i8(-0x1i8);
        let ret: [u8; 2] = [b'b' as u8, -0x1i8 as u8];
        buf.clear();
        value.encode(buf.borrow_mut());
        assert_eq!(&buf[..], ret);

        let value = FieldValue::from_u8(0x1u8);
        let ret = [b'B', 0x1u8];
        buf.clear();
        value.encode(buf.borrow_mut());
        assert_eq!(&buf[..], ret);

        let value = FieldValue::from_i16(-0x1234i16);
        let mut ret = BytesMut::with_capacity(8);
        ret.put_u8(b's');
        ret.put_i16(-0x1234i16);
        buf.clear();
        value.encode(buf.borrow_mut());
        assert_eq!(&buf[..], &ret[..]);

        let value = FieldValue::from_u16(0x1234u16);
        let mut ret = BytesMut::with_capacity(8);
        ret.put_u8(b'u');
        ret.put_u16(0x1234u16);
        buf.clear();
        value.encode(buf.borrow_mut());
        assert_eq!(&buf[..], &ret[..]);

        let value = FieldValue::from_i32(-0x1i32);
        let mut ret = BytesMut::with_capacity(32);
        ret.put_u8(b'I');
        ret.put_i32(-0x1i32);
        buf.clear();
        value.encode(buf.borrow_mut());
        assert_eq!(&buf[..], &ret[..]);

        let value = FieldValue::from_u32(0x12345678u32);
        let mut ret = BytesMut::with_capacity(8);
        ret.put_u8(b'i');
        ret.put_u32(0x12345678u32);
        buf.clear();
        value.encode(buf.borrow_mut());
        assert_eq!(&buf[..], &ret[..]);

        let value = FieldValue::from_i64(-0x12345678i64);
        let mut ret = BytesMut::with_capacity(64);
        ret.put_u8(b'l');
        ret.put_i64(-0x12345678i64);
        buf.clear();
        value.encode(buf.borrow_mut());
        assert_eq!(&buf[..], &ret[..]);

        let value = FieldValue::from_u64(0x12345678u64);
        let mut ret = BytesMut::with_capacity(8);
        ret.put_u8(b'L');
        ret.put_u64(0x12345678u64);
        buf.clear();
        value.encode(buf.borrow_mut());
        assert_eq!(&buf[..], &ret[..]);

        let value = FieldValue::from_f32(12345678.12f32);
        let mut ret = BytesMut::with_capacity(64);
        ret.put_u8(b'f');
        ret.put_f32(12345678.12f32);
        buf.clear();
        value.encode(buf.borrow_mut());
        assert_eq!(&buf[..], &ret[..]);

        let value = FieldValue::from_f64(12345678.12f64);
        let mut ret = BytesMut::with_capacity(64);
        ret.put_u8(b'd');
        ret.put_f64(12345678.12f64);
        buf.clear();
        value.encode(buf.borrow_mut());
        assert_eq!(&buf[..], &ret[..]);

        let value = FieldValue::from_timestamp(0x12345678u64);
        let mut ret = BytesMut::with_capacity(8);
        ret.put_u8(b'T');
        ret.put_u64(0x12345678u64);
        buf.clear();
        value.encode(buf.borrow_mut());
        assert_eq!(&buf[..], &ret[..]);

        let value = FieldValue::from_decimal(Decimal::new(2, 0x12345678u32));
        let mut ret = BytesMut::with_capacity(8);
        ret.put_u8(b'D');
        ret.put_u8(2u8);
        ret.put_u32(0x12345678u32);
        buf.clear();
        value.encode(buf.borrow_mut());
        assert_eq!(&buf[..], &ret[..]);

        let value = FieldValue::from_long_string(LongStr::with_bytes(b"hello").unwrap());
        let mut ret = BytesMut::with_capacity(8);
        ret.put_u8(b'S');
        ret.put_u32(0x5u32);
        ret.put_slice(b"hello");
        buf.clear();
        value.encode(buf.borrow_mut());
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
        value.encode(buf.borrow_mut());
        assert_eq!(&buf[..], &ret[..]);

        let mut table = FieldTable::new();
        table.insert(FieldName::with_bytes(b"hello").unwrap(), FieldValue::from_u32(0x12345678u32));
        table.insert(FieldName::with_bytes(b"world").unwrap(), FieldValue::from_long_string(LongStr::with_bytes(b"hello").unwrap()));
        let mut ret = BytesMut::with_capacity(128);
        ret.put_u8(b'F');
        ret.put_u32(27u32);
        for (k, _) in &table {
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
        value.encode(buf.borrow_mut());
        assert_eq!(&buf[..], &ret[..]);

        let value = FieldValue::from_bytes_array(LongStr::with_bytes(b"hello").unwrap());
        let mut ret = BytesMut::with_capacity(8);
        ret.put_u8(b'x');
        ret.put_u32(0x5u32);
        ret.put_slice(b"hello");
        buf.clear();
        value.encode(buf.borrow_mut());
        assert_eq!(&buf[..], &ret[..]);

        let value = FieldValue::from_void();
        let ret = [b'V'];
        buf.clear();
        value.encode(buf.borrow_mut());
        assert_eq!(&buf[..], ret)
    }
}
