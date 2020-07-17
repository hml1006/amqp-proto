use property::Property;
use bytes::{BytesMut, BufMut};
use crate::{ShortStr, FieldTable, LongStr};
use crate::common::{WriteToBuf, MethodId, Class, Method};

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ConnectionStart {
    version_major: u8,
    version_minor: u8,
    server_properties: FieldTable,
    mechanisms: LongStr,
    locales: LongStr,
}

impl WriteToBuf for ConnectionStart {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u8(self.version_major);
        buffer.put_u8(self.version_minor);
        self.server_properties.write_to_buf(buffer);
        self.mechanisms.write_to_buf(buffer);
        self.locales.write_to_buf(buffer);
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ConnectionStartOk {
    client_properties: FieldTable,
    mechanism: ShortStr,
    response: LongStr,
    locale: ShortStr
}

impl WriteToBuf for ConnectionStartOk {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        self.client_properties.write_to_buf(buffer);
        self.mechanism.write_to_buf(buffer);
        self.response.write_to_buf(buffer);
        self.locale.write_to_buf(buffer);
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ConnectionSecure {
    challenge: LongStr
}

impl WriteToBuf for ConnectionSecure {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        self.challenge.write_to_buf(buffer);
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ConnectionSecureOk {
    response: LongStr
}

impl WriteToBuf for ConnectionSecureOk {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        self.response.write_to_buf(buffer);
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ConnectionTune {
    channel_max: u16,
    frame_max: u32,
    heartbeat: u16
}

impl WriteToBuf for ConnectionTune {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u16(self.channel_max);
        buffer.put_u32(self.frame_max);
        buffer.put_u16(self.heartbeat);
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ConnectionTuneOk {
    channel_max: u16,
    frame_max: u32,
    heartbeat: u16
}

impl WriteToBuf for ConnectionTuneOk {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u16(self.channel_max);
        buffer.put_u32(self.frame_max);
        buffer.put_u16(self.heartbeat);
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ConnectionOpen {
    vhost: ShortStr,
    capabilities: ShortStr,  // rabbitmq used
    insist: bool
}

impl WriteToBuf for ConnectionOpen {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        self.vhost.write_to_buf(buffer);
        self.capabilities.write_to_buf(buffer);
        buffer.put_u8(if self.insist {1u8} else {0u8});
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ConnectionOpenOk {
    known_hosts: ShortStr
}

impl WriteToBuf for ConnectionOpenOk {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        self.known_hosts.write_to_buf(buffer);
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ConnectionClose {
    reply_code: u16,
    reply_text: ShortStr,
    class: Class,
    method: Method
}

impl WriteToBuf for ConnectionClose {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u16(self.reply_code);
        self.reply_text.write_to_buf(buffer);
        buffer.put_u16(self.class.class_id());
        buffer.put_u16(self.method.method_id())
    }
}

pub struct ConnectionCloseOk;

impl WriteToBuf for ConnectionCloseOk {
    #[inline]
    fn write_to_buf(&self, _: &mut BytesMut) {
    }
}


#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ConnectionProperties {
    flags: u32,
}

impl WriteToBuf for ConnectionProperties {
    #[inline]
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u32(self.flags);
    }
}


pub enum ConnectionMethod {
    Start,
    StartOk,
    Secure,
    SecureOk,
    Tune,
    TuneOk,
    Open,
    OpenOk,
    Close,
    CloseOk,
    Unknown
}

impl MethodId for ConnectionMethod {
    fn method_id(&self) -> u16 {
        match self {
            ConnectionMethod::Start => 10,
            ConnectionMethod::StartOk => 11,
            ConnectionMethod::Secure => 20,
            ConnectionMethod::SecureOk => 21,
            ConnectionMethod::Tune => 30,
            ConnectionMethod::TuneOk => 31,
            ConnectionMethod::Open => 40,
            ConnectionMethod::OpenOk => 41,
            ConnectionMethod::Close => 50,
            ConnectionMethod::CloseOk => 51,
            ConnectionMethod::Unknown => 0xffff
        }
    }
}

impl Default for ConnectionMethod {
    fn default() -> Self {
        ConnectionMethod::Unknown
    }
}

impl From<u16> for ConnectionMethod {
    fn from(method_id: u16) -> Self {
        match method_id {
            10 => ConnectionMethod::Start,
            11 => ConnectionMethod::StartOk,
            20 => ConnectionMethod::Secure,
            21 => ConnectionMethod::SecureOk,
            30 => ConnectionMethod::Tune,
            31 => ConnectionMethod::TuneOk,
            40 => ConnectionMethod::Open,
            41 => ConnectionMethod::OpenOk,
            50 => ConnectionMethod::Close,
            51 => ConnectionMethod::CloseOk,
            _  => ConnectionMethod::Unknown
        }
    }
}