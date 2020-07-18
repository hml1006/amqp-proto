use property::Property;
use bytes::{BytesMut, BufMut};
use crate::{ShortStr, LongStr};
use crate::common::{Encode, MethodId, Class, Method, Decode, get_method_type};
use crate::frame::{Arguments, Property};
use crate::error::FrameDecodeErr;

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ChannelOpen {
    out_of_band: ShortStr
}

impl Encode for ChannelOpen {
    #[inline]
    fn encode(&self, buffer: &mut BytesMut) {
        self.out_of_band.encode(buffer);
    }
}

impl Decode<Arguments> for ChannelOpen {
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        let (buffer, out_of_band) = match ShortStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        Ok((buffer, Arguments::ChannelOpen(ChannelOpen { out_of_band })))
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ChannelOpenOk {
    channel_id: LongStr
}

impl Encode for ChannelOpenOk {
    fn encode(&self, buffer: &mut BytesMut) {
        self.channel_id.encode(buffer);
    }
}

impl Decode<Arguments> for ChannelOpenOk {
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        let (buffer, channel_id) = match LongStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        Ok((buffer, Arguments::ChannelOpenOk(ChannelOpenOk { channel_id })))
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ChannelFlow {
    active: bool
}

impl Encode for ChannelFlow {
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.put_u8(if self.active { 1 } else { 0})
    }
}

impl Decode<Arguments> for ChannelFlow {
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        let (buffer, flags) = match u8::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        let active = if flags & (1 << 0) != 0 { true } else { false };
        Ok((buffer, Arguments::ChannelFlow(ChannelFlow { active })))
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ChannelFlowOk {
    active: bool
}

impl Encode for ChannelFlowOk {
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.put_u8(if self.active { 1 } else { 0})
    }
}

impl Decode<Arguments> for ChannelFlowOk {
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        let (buffer, flags) = match u8::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        let active = if flags & (1 << 0) != 0 { true } else { false };
        Ok((buffer, Arguments::ChannelFlowOk(ChannelFlowOk { active })))
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ChannelClose {
    reply_code: u16,
    reply_text: ShortStr,
    class: Class,
    method: Method
}

impl Encode for ChannelClose {
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.put_u16(self.reply_code);
        self.reply_text.encode(buffer);
        buffer.put_u16(self.class.class_id());
        buffer.put_u16(self.method.method_id());
    }
}

impl Decode<Arguments> for ChannelClose {
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
        Ok((buffer, Arguments::ChannelClose(ChannelClose { reply_code, reply_text, class, method })))
    }
}

pub struct ChannelCloseOk;

impl Encode for ChannelCloseOk {
    #[inline]
    fn encode(&self, _: &mut BytesMut) {
    }
}

impl Decode<Arguments> for ChannelCloseOk {
    #[inline]
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        Ok((buffer, Arguments::ChannelCloseOk(ChannelCloseOk)))
    }
}

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

pub enum ChannelMethod {
    Open,
    OpenOk,
    Flow,
    FlowOk,
    Close,
    CloseOk,
    Unknown
}

impl MethodId for ChannelMethod {
    fn method_id(&self) -> u16 {
        match self {
            ChannelMethod::Open => 10,
            ChannelMethod::OpenOk => 11,
            ChannelMethod::Flow => 20,
            ChannelMethod::FlowOk => 21,
            ChannelMethod::Close => 40,
            ChannelMethod::CloseOk => 41,
            ChannelMethod::Unknown => 0xffff
        }
    }
}

impl Default for ChannelMethod {
    fn default() -> ChannelMethod {
        ChannelMethod::Unknown
    }
}

impl From<u16> for ChannelMethod {
    fn from(method_id: u16) -> Self {
        match method_id {
            10 => ChannelMethod::Open,
            11 => ChannelMethod::OpenOk,
            20 => ChannelMethod::Flow,
            21 => ChannelMethod::FlowOk,
            40 => ChannelMethod::Close,
            41 => ChannelMethod::CloseOk,
            _  => ChannelMethod::Unknown
        }
    }
}