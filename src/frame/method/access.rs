use property::Property;
use bytes::{BytesMut, BufMut};
use crate::frame::base::{ShortStr, Encode, Arguments, Decode};
use crate::error::FrameDecodeErr;

// Accesss is deprecated in amqp0-9-1, this is just for compatibility
#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct AccessRequest {
    realm: ShortStr,
    exclusive: bool,
    passive: bool,
    active: bool,
    write: bool,
    read: bool
}

impl Encode for AccessRequest {
    #[inline]
    fn encode(&self, buffer: &mut BytesMut) {
        self.realm.encode(buffer);
        // just fill 0
        buffer.put_u8(0);
    }
}

impl Decode<Arguments> for AccessRequest {
    #[inline]
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr> {
        let (buffer, realm) = match ShortStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode AccessRequest realm -> {}", e)))
        };
        let (_, _) = match u8::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode AccessRequest flags -> {}", e)))
        };
        Ok((buffer, Arguments::AccessRequest(AccessRequest { realm, exclusive: false, passive: false, active: false, write: false, read: false })))
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct AccessRequestOk {
    ticket: u16
}

impl Encode for AccessRequestOk {
    #[inline]
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.put_u16(self.ticket);
    }
}

impl Decode<Arguments> for AccessRequestOk {
    #[inline]
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr> {
        let (_, ticket) = match u16::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode AccessRequestOk ticket -> {}", e)))
        };
        Ok((buffer, Arguments::AccessRequestOk(AccessRequestOk { ticket })))
    }
}
