use property::Property;
use bytes::{BytesMut, BufMut};
use crate::{ShortStr, LongStr};
use crate::common::{WriteToBuf, MethodId, Class, Method};

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ChannelOpen {
    out_of_band: ShortStr
}

impl WriteToBuf for ChannelOpen {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        self.out_of_band.write_to_buf(buffer);
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ChannelOpenOk {
    channel_id: LongStr
}

impl WriteToBuf for ChannelOpenOk {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        self.channel_id.write_to_buf(buffer);
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ChannelFlow {
    active: bool
}

impl WriteToBuf for ChannelFlow {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u8(if self.active { 1 } else { 0})
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ChannelFlowOk {
    active: bool
}

impl WriteToBuf for ChannelFlowOk {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u8(if self.active { 1 } else { 0})
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

impl WriteToBuf for ChannelClose {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u16(self.reply_code);
        self.reply_text.write_to_buf(buffer);
        buffer.put_u16(self.class.class_id());
        buffer.put_u16(self.method.method_id());
    }
}

pub struct ChannelCloseOk;

impl WriteToBuf for ChannelCloseOk {
    #[inline]
    fn write_to_buf(&self, _: &mut BytesMut) {
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ChannelProperties {
    flags: u32,
}

impl WriteToBuf for ChannelProperties {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u32(self.flags);
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