use property::Property;
use bytes::{BytesMut, BufMut};
use crate::ShortStr;
use crate::common::{WriteToBuf, MethodId};

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

impl WriteToBuf for AccessRequest {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        self.realm.write_to_buf(buffer);
        // just fill 0
        buffer.put_u8(0);
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct AccessRequestOk {
    ticket: u16
}

impl WriteToBuf for AccessRequestOk {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u16(self.ticket);
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct AccessProperties {
    flags: u32,
}

impl WriteToBuf for AccessProperties {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u32(self.flags);
    }
}


pub enum AccessMethod {
    Request,
    RequestOk,
    Unknown
}

impl MethodId for AccessMethod {
    fn method_id(&self) -> u16 {
        match self {
            AccessMethod::Request => 10,
            AccessMethod::RequestOk => 11,
            AccessMethod::Unknown => 0xffff
        }
    }
}

impl Default for AccessMethod {
    fn default() -> Self {
        AccessMethod::Unknown
    }
}

impl From<u16> for AccessMethod {
    fn from(method_id: u16) -> Self {
        match method_id {
            10 => AccessMethod::Request,
            11 => AccessMethod::RequestOk,
            _  => AccessMethod::Unknown
        }
    }
}