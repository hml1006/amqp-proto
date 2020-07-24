use property::Property;
use crate::frame::base::{Encode, Decode, Property};
use bytes::{BytesMut, BufMut};
use crate::error::FrameDecodeErr;

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct QueueProperties {
    flags: u32,
}

impl Encode for QueueProperties {
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.put_u32(self.flags);
    }
}

impl Decode<Property> for QueueProperties {
    fn decode(buffer: &[u8]) -> Result<(&[u8], Property), FrameDecodeErr>{
        let (buffer, flags) = match u32::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        Ok((buffer, Property::Queue(QueueProperties { flags })))
    }
}
