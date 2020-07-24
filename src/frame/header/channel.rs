use property::Property;
use crate::frame::base::{Encode, Property, Decode};
use bytes::{BytesMut, BufMut};
use crate::error::FrameDecodeErr;

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ChannelProperties {
    flags: u32,
}

impl Encode for ChannelProperties {
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.put_u32(self.flags);
    }
}

impl Decode<Property> for ChannelProperties {
    fn decode(buffer: &[u8]) -> Result<(&[u8], Property), FrameDecodeErr>{
        let (buffer, flags) = match u32::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        Ok((buffer, Property::Channel(ChannelProperties { flags })))
    }
}
