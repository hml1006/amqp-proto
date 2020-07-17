use property::Property;
use bytes::{BytesMut, BufMut};
use crate::common::{WriteToBuf, MethodId};

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ConfirmSelect {
    no_wait: bool
}

impl WriteToBuf for ConfirmSelect {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u8(if self.no_wait { 1 } else { 0 });
    }
}

pub struct ConfirmSelectOk;

impl WriteToBuf for ConfirmSelectOk {
    #[inline]
    fn write_to_buf(&self, _: &mut BytesMut) {
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ConfirmProperties {
    flags: u32,
}

impl WriteToBuf for ConfirmProperties {
    #[inline]
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u32(self.flags);
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