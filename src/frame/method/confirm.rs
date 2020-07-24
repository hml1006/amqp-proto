use property::Property;
use bytes::{BytesMut, BufMut};
use crate::error::FrameDecodeErr;
use crate::frame::base::{Arguments, Decode, Encode};

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ConfirmSelect {
    no_wait: bool
}

impl Encode for ConfirmSelect {
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.put_u8(if self.no_wait { 1 } else { 0 });
    }
}

impl Decode<Arguments> for ConfirmSelect {
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        let (buffer, flags) = match u8::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode ConfirmSelect flags -> {}", e)))
        };
        let no_wait = if flags & (1 << 0) != 0 { true } else { false };
        Ok((buffer, Arguments::ConfirmSelect(ConfirmSelect { no_wait })))
    }
}

pub struct ConfirmSelectOk;

impl Encode for ConfirmSelectOk {
    #[inline]
    fn encode(&self, _: &mut BytesMut) {
    }
}

impl Decode<Arguments> for ConfirmSelectOk {
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        Ok((buffer, Arguments::ConfirmSelectOk(ConfirmSelectOk)))
    }
}
