use property::Property;
use bytes::{BytesMut, BufMut};
use crate::ShortStr;
use crate::common::{Encode, MethodId, Decode};
use crate::error::FrameDecodeErr;
use crate::frame::{Arguments, Property};

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
            Err(e) => return Err(e)
        };
        let (_, _) = match u8::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
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
            Err(e) => return Err(e)
        };
        Ok((buffer, Arguments::AccessRequestOk(AccessRequestOk { ticket })))
    }
}

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
            Err(e) => return Err(e)
        };
        Ok((buffer, Property::Access(AccessProperties { flags })))
    }
}

pub enum AccessMethod {
    Request,
    RequestOk,
    Unknown
}

impl MethodId for AccessMethod {
    #[inline]
    fn method_id(&self) -> u16 {
        match self {
            AccessMethod::Request => 10,
            AccessMethod::RequestOk => 11,
            AccessMethod::Unknown => 0xffff
        }
    }
}

impl Default for AccessMethod {
    #[inline]
    fn default() -> Self {
        AccessMethod::Unknown
    }
}

impl From<u16> for AccessMethod {
    #[inline]
    fn from(method_id: u16) -> Self {
        match method_id {
            10 => AccessMethod::Request,
            11 => AccessMethod::RequestOk,
            _  => AccessMethod::Unknown
        }
    }
}