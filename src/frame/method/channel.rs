use property::Property;
use bytes::{BytesMut, BufMut};
use crate::error::FrameDecodeErr;
use crate::frame::base::{ShortStr, Encode, Arguments, Decode};
use crate::class::Class;
use crate::LongStr;
use crate::method::{Method, get_method_type, MethodId};

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
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode ChannelOpen out_of_band -> {}", e)))
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
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode ChannelOpenOk channel_id -> {}", e)))
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
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode ChannelFlow flags -> {}", e)))
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
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode ChannelFlowOk flags -> {}", e)))
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
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode ChannelClose reply_code -> {}", e)))
        };
        let (buffer, reply_text) = match ShortStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode ChannelClose reply_text -> {}", e)))
        };
        let (buffer, class_id) = match u16::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode ChannelClose class_id -> {}", e)))
        };
        let (buffer, method_id) = match u16::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode ChannelClose method_id -> {}", e)))
        };
        let class = Class::from(class_id);
        if let Class::Unknown = class {
            return Err(FrameDecodeErr::SyntaxError("decode ChannelClose class unknown"));
        }
        let method = match get_method_type(class.clone(), method_id) {
            Ok(method) => method,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode ChannelClose method -> {}", e)))
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
