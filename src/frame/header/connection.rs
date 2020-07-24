use property::Property;
use crate::frame::base::{Encode, Decode, Property};
use bytes::{BytesMut, BufMut};
use crate::error::FrameDecodeErr;

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ConnectionProperties {
    flags: u32,
}

impl Encode for ConnectionProperties {
    #[inline]
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.put_u32(self.flags);
    }
}

impl Decode<Property> for ConnectionProperties {
    fn decode(buffer: &[u8]) -> Result<(&[u8], Property), FrameDecodeErr>{
        let (buffer, flags) = match u32::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        Ok((buffer, Property::Connection(ConnectionProperties { flags })))
    }
}
