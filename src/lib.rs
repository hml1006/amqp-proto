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

/// Decode and Encode frame, also has an tokio frame codec.
pub mod codec {
    pub use crate::frame::frame_codec::{DecodedFrame, FrameCodec};
    pub use crate::frame::base::{ContentHeaderPayload, HeartbeatPayload, MethodPayload, Payload, Frame, ProtocolHeader, Decode, Encode};
}

/// Frame decode error and amqp protocol error definitions.
pub mod err {
    pub use crate::error::FrameDecodeErr;
    pub use crate::error::amqp::{AmqpError, AmqpErrorKind};
}

#[cfg(test)]
mod tests {
    use crate::{LongStr, FieldValue, FieldTable, FieldName};
    use crate::frame::method::connection::ConnectionStart;
    use crate::codec::{Decode};
    use bytes::{BytesMut, BufMut};

    #[test]
    fn test_connection_start() {
        let mut connection_start = ConnectionStart::default();
        connection_start.set_version_major(0);
        connection_start.set_version_minor(9);

        assert_eq!(connection_start.version_major(), 0);
    }

    #[test]
    fn test_field_table() {
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
        if let (_, FieldValue::FieldTable(t)) = FieldValue::decode(&ret).unwrap() {
            assert!(matches!(t.get(&FieldName::with_bytes(b"hello").unwrap()).unwrap(), FieldValue::U32(v) if *v == 0x12345678u32));
            assert!(matches!(t.get(&FieldName::with_bytes(b"world").unwrap()).unwrap(), FieldValue::LongStr(v) if v.to_string() == String::from("hello")));
        } else {
            panic!("Expected FieldTable value");
        }
    }
}
