use property::Property;
use bytes::{BytesMut, BufMut};
use crate::common::{Encode, MethodId, Decode};
use crate::frame::{Arguments, Property};
use crate::error::FrameDecodeErr;

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
            Err(e) => return Err(e)
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

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ConfirmProperties {
    flags: u32,
}

impl Encode for ConfirmProperties {
    #[inline]
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.put_u32(self.flags);
    }
}

impl Decode<Property> for ConfirmProperties {
    fn decode(buffer: &[u8]) -> Result<(&[u8], Property), FrameDecodeErr>{
        let (buffer, flags) = match u32::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        Ok((buffer, Property::Confirm(ConfirmProperties { flags })))
    }
}

pub enum ConfirmMethod {
    Select,
    SelectOk,
    Unknown
}

impl MethodId for ConfirmMethod {
    fn method_id(&self) -> u16 {
        match self {
            ConfirmMethod::Select => 10,
            ConfirmMethod::SelectOk => 11,
            ConfirmMethod::Unknown => 0xffff
        }
    }
}

impl Default for ConfirmMethod {
    fn default() -> Self {
        ConfirmMethod::Unknown
    }
}

impl From<u16> for ConfirmMethod {
    fn from(method_id: u16) -> Self {
        match method_id {
            10 => ConfirmMethod::Select,
            11 => ConfirmMethod::SelectOk,
            _ => ConfirmMethod::Unknown
        }
    }
}