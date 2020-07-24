use bytes::{BufMut, BytesMut};
use property::Property;
use crate::frame::base::{Encode, Property, Decode};
use crate::error::FrameDecodeErr;

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct AccessProperties {
    flags: u32,
}

impl Encode for AccessProperties {
    #[inline]
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.put_u32(self.flags);
    }
}

impl Decode<Property> for AccessProperties {
    #[inline]
    fn decode(buffer: &[u8]) -> Result<(&[u8], Property), FrameDecodeErr>{
        let (buffer, flags) = match u32::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode AccessProperties flags -> {}", e)))
        };
        Ok((buffer, Property::Access(AccessProperties { flags })))
    }
}