use property::Property;
use bytes::{BytesMut, BufMut};
use crate::{ShortStr, FieldTable, LongStr};
use crate::common::{Encode, MethodId, Class, Method, Decode, get_method_type};
use crate::frame::{Arguments, Property};
use crate::error::FrameDecodeErr;

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ConnectionStart {
    version_major: u8,
    version_minor: u8,
    server_properties: FieldTable,
    mechanisms: LongStr,
    locales: LongStr,
}

impl Encode for ConnectionStart {
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.put_u8(self.version_major);
        buffer.put_u8(self.version_minor);
        self.server_properties.encode(buffer);
        self.mechanisms.encode(buffer);
        self.locales.encode(buffer);
    }
}

impl Decode<Arguments> for ConnectionStart {
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        let (buffer, version_major) = match u8::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        let (buffer, version_minor) = match u8::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        let (buffer, server_properties) = match FieldTable::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        let (buffer, mechanisms) = match LongStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        let (buffer, locales) = match LongStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        Ok((buffer, Arguments::ConnectionStart(ConnectionStart { version_major, version_minor, server_properties, mechanisms, locales })))
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

impl Encode for ConnectionStartOk {
    fn encode(&self, buffer: &mut BytesMut) {
        self.client_properties.encode(buffer);
        self.mechanism.encode(buffer);
        self.response.encode(buffer);
        self.locale.encode(buffer);
    }
}

impl Decode<Arguments> for ConnectionStartOk {
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        let (buffer, client_properties) = match FieldTable::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        let (buffer, mechanism) = match ShortStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        let (buffer, response) = match LongStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        let (buffer, locale) = match ShortStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        Ok((buffer, Arguments::ConnectionStartOk(ConnectionStartOk { client_properties, mechanism, response, locale })))
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ConnectionSecure {
    challenge: LongStr
}

impl Encode for ConnectionSecure {
    fn encode(&self, buffer: &mut BytesMut) {
        self.challenge.encode(buffer);
    }
}

impl Decode<Arguments> for ConnectionSecure {
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        let (buffer, challenge) = match LongStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        Ok((buffer, Arguments::ConnectionSecure(ConnectionSecure { challenge })))
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ConnectionSecureOk {
    response: LongStr
}

impl Encode for ConnectionSecureOk {
    fn encode(&self, buffer: &mut BytesMut) {
        self.response.encode(buffer);
    }
}

impl Decode<Arguments> for ConnectionSecureOk {
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        let (buffer, response) = match LongStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        Ok((buffer, Arguments::ConnectionSecureOk(ConnectionSecureOk { response })))
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ConnectionTune {
    channel_max: u16,
    frame_max: u32,
    heartbeat: u16
}

impl Encode for ConnectionTune {
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.put_u16(self.channel_max);
        buffer.put_u32(self.frame_max);
        buffer.put_u16(self.heartbeat);
    }
}

impl Decode<Arguments> for ConnectionTune {
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        let (buffer, channel_max) = match u16::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        let (buffer, frame_max) = match u32::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        let (buffer, heartbeat) = match u16::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        Ok((buffer, Arguments::ConnectionTune(ConnectionTune { channel_max, frame_max, heartbeat })))
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ConnectionTuneOk {
    channel_max: u16,
    frame_max: u32,
    heartbeat: u16
}

impl Encode for ConnectionTuneOk {
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.put_u16(self.channel_max);
        buffer.put_u32(self.frame_max);
        buffer.put_u16(self.heartbeat);
    }
}

impl Decode<Arguments> for ConnectionTuneOk {
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        let (buffer, channel_max) = match u16::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        let (buffer, frame_max) = match u32::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        let (buffer, heartbeat) = match u16::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        Ok((buffer, Arguments::ConnectionTuneOk(ConnectionTuneOk { channel_max, frame_max, heartbeat })))
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ConnectionOpen {
    vhost: ShortStr,
    capabilities: ShortStr,  // rabbitmq used
    insist: bool
}

impl Encode for ConnectionOpen {
    fn encode(&self, buffer: &mut BytesMut) {
        self.vhost.encode(buffer);
        self.capabilities.encode(buffer);
        buffer.put_u8(if self.insist {1u8} else {0u8});
    }
}

impl Decode<Arguments> for ConnectionOpen {
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        let (buffer, vhost) = match ShortStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        let (buffer, capabilities) = match ShortStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        let (buffer, flags) = match u8::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        let insist = if flags & (1 << 0) != 0 { true } else { false };
        Ok((buffer, Arguments::ConnectionOpen(ConnectionOpen { vhost, capabilities, insist })))
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ConnectionOpenOk {
    known_hosts: ShortStr
}

impl Encode for ConnectionOpenOk {
    fn encode(&self, buffer: &mut BytesMut) {
        self.known_hosts.encode(buffer);
    }
}

impl Decode<Arguments> for ConnectionOpenOk {
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        let (buffer, known_hosts) = match ShortStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        Ok((buffer, Arguments::ConnectionOpenOk(ConnectionOpenOk { known_hosts })))
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

impl Encode for ConnectionClose {
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.put_u16(self.reply_code);
        self.reply_text.encode(buffer);
        buffer.put_u16(self.class.class_id());
        buffer.put_u16(self.method.method_id())
    }
}

impl Decode<Arguments> for ConnectionClose {
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        let (buffer, reply_code) = match u16::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        let (buffer, reply_text) = match ShortStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        let (buffer, class_id) = match u16::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        let (buffer, method_id) = match u16::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        let class = Class::from(class_id);
        if let Class::Unknown = class {
            return Err(FrameDecodeErr::UnknownClassType);
        }
        let method = match get_method_type(class.clone(), method_id) {
            Ok(method) => method,
            Err(e) => return Err(e)
        };
        Ok((buffer, Arguments::ConnectionClose(ConnectionClose { reply_code, reply_text, class, method })))
    }
}

pub struct ConnectionCloseOk;

impl Encode for ConnectionCloseOk {
    #[inline]
    fn encode(&self, _: &mut BytesMut) {
    }
}

impl Decode<Arguments> for ConnectionCloseOk {
    #[inline]
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        Ok((buffer, Arguments::ConnectionCloseOk(ConnectionCloseOk)))
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ConnectionProperties {
    flags: u32,
}

impl Encode for ConnectionProperties {
    #[inline]
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.put_u32(self.flags);
    }
}

impl Decode<Property> for ConnectionProperties {
    fn decode(buffer: &[u8]) -> Result<(&[u8], Property), FrameDecodeErr>{
        let (buffer, flags) = match u32::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        Ok((buffer, Property::Connection(ConnectionProperties { flags })))
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