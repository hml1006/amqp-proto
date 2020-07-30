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
    use crate::{LongStr, Decimal, FieldValue, FieldTable, FieldName, FieldArray};
    use crate::frame::method::connection::ConnectionStart;
    use crate::codec::{Encode, Decode};
    use bytes::{BytesMut, BufMut};
    use std::borrow::BorrowMut;

    #[test]
    fn test_connection_start() {
        let mut connection_start = ConnectionStart::default();
        connection_start.set_version_major(0);
        connection_start.set_version_minor(9);

        assert_eq!(connection_start.version_major(), 0);
    }

}
