use property::Property;
use bytes::{BytesMut, BufMut};
use crate::{ShortStr, FieldTable};
use crate::common::{Encode, MethodId, Decode};
use crate::frame::{Arguments, Property};
use crate::error::FrameDecodeErr;

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct QueueDeclare {
    ticket: u16,
    queue_name: ShortStr,
    passive: bool,
    durable: bool,
    exclusive: bool,
    auto_delete: bool,
    no_wait: bool,
    args: FieldTable
}

impl Encode for QueueDeclare {
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.put_u16(self.ticket);
        self.queue_name.encode(buffer);
        let mut flag = 0u8;
        flag |= if self.passive { 1 } else { 0 };
        flag |= if self.durable { 1 << 1 } else { 0};
        flag |= if self.exclusive { 1 << 2 } else { 0 };
        flag |= if self.auto_delete { 1 << 3 } else { 0 };
        flag |= if self.no_wait { 1 << 4 } else { 0 };
        buffer.put_u8(flag);
        self.args.encode(buffer);
    }
}

impl Decode<Arguments> for QueueDeclare {
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        let (buffer, ticket) = match u16::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        let (buffer, queue_name) = match ShortStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        let (buffer, flags) = match u8::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        let (buffer, args) = match FieldTable::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        let passive = if flags & (1 << 0) != 0 { true } else { false };
        let durable = if flags & (1 << 1) != 0 { true } else { false };
        let exclusive = if flags & (1 << 2) != 0 { true } else { false };
        let auto_delete = if flags & (1 << 3) != 0 { true } else { false };
        let no_wait = if flags & (1 << 4) != 0 { true } else { false };
        Ok((buffer, Arguments::QueueDeclare(QueueDeclare { ticket, queue_name, passive, durable, exclusive, auto_delete, no_wait, args})))
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct QueueDeclareOk {
    queue_name: ShortStr,
    message_count: u32,
    consumer_count: u32
}

impl Encode for QueueDeclareOk {
    fn encode(&self, buffer: &mut BytesMut) {
        self.queue_name.encode(buffer);
        buffer.put_u32(self.message_count);
        buffer.put_u32(self.consumer_count);
    }
}

impl Decode<Arguments> for QueueDeclareOk {
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        let (buffer, queue_name) = match ShortStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        let (buffer, message_count) = match u32::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        let (buffer, consumer_count) = match u32::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        Ok((buffer, Arguments::QueueDeclareOk(QueueDeclareOk {queue_name, message_count, consumer_count})))
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct QueueBind {
    ticket: u16,
    queue_name: ShortStr,
    exchange_name: ShortStr,
    routing_key: ShortStr,
    no_wait: bool,
    args: FieldTable
}

impl Encode for QueueBind {
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.put_u16(self.ticket);
        self.queue_name.encode(buffer);
        self.exchange_name.encode(buffer);
        self.routing_key.encode(buffer);
        buffer.put_u8(if self.no_wait { 1 } else { 0 });
        self.args.encode(buffer);
    }
}

impl Decode<Arguments> for QueueBind {
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        let (buffer, ticket) = match u16::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        let (buffer, queue_name) = match ShortStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        let (buffer, exchange_name) = match ShortStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        let (buffer, routing_key) = match ShortStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        let (buffer, flags) = match u8::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        let (buffer, args) = match FieldTable::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        let no_wait = if flags & (1 << 0) != 0 { true } else { false };
        Ok((buffer, Arguments::QueueBind(QueueBind {ticket, queue_name, exchange_name, routing_key, no_wait, args})))
    }
}

pub struct QueueBindOk;

impl Encode for QueueBindOk {
    #[inline]
    fn encode(&self, _: &mut BytesMut) {
    }
}

impl Decode<Arguments> for QueueBindOk {
    #[inline]
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        Ok((buffer, Arguments::QueueBindOk(QueueBindOk)))
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct QueuePurge {
    ticket: u16,
    queue_name: ShortStr,
    no_wait: bool
}

impl Encode for QueuePurge {
    #[inline]
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.put_u16(self.ticket);
        self.queue_name.encode(buffer);
        buffer.put_u8(if self.no_wait { 1 } else { 0 });
    }
}

impl Decode<Arguments> for QueuePurge {
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        let (buffer, ticket) = match u16::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        let (buffer, queue_name) = match ShortStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        let (buffer, flags) = match u8::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        let no_wait = if flags & (1 << 0) != 0 { true } else { false };
        Ok((buffer, Arguments::QueuePurge(QueuePurge { ticket, queue_name, no_wait})))
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct QueuePurgeOk {
    message_count: u32
}

impl Encode for QueuePurgeOk {
    #[inline]
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.put_u32(self.message_count);
    }
}

impl Decode<Arguments> for QueuePurgeOk {
    #[inline]
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        let (buffer, message_count) = match u32::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        Ok((buffer, Arguments::QueuePurgeOk(QueuePurgeOk { message_count })))
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct QueueDelete {
    ticket: u16,
    queue_name: ShortStr,
    if_unused: bool,
    if_empty: bool,
    no_wait: bool
}

impl Encode for QueueDelete {
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.put_u16(self.ticket);
        self.queue_name.encode(buffer);
        let mut flag = 0u8;
        flag |= if self.if_unused { 1 } else { 0};
        flag |= if self.if_empty { 1 << 1 } else { 0 };
        flag |= if self.no_wait { 1 << 2 } else { 0 };
        buffer.put_u8(flag);
    }
}

impl Decode<Arguments> for QueueDelete {
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        let (buffer, ticket) = match u16::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        let (buffer, queue_name) = match ShortStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        let (buffer, flags) = match u8::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        let if_unused = if flags & (1 << 0) != 0 { true } else { false };
        let if_empty = if flags & (1 << 1) != 0 { true } else { false };
        let no_wait = if flags & (1 << 2) != 0 { true } else { false };
        Ok((buffer, Arguments::QueueDelete(QueueDelete { ticket, queue_name, if_unused, if_empty, no_wait})))
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct QueueDeleteOk {
    message_count: u32
}

impl Encode for QueueDeleteOk {
    #[inline]
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.put_u32(self.message_count);
    }
}

impl Decode<Arguments> for QueueDeleteOk {
    #[inline]
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        let (buffer, message_count) = match u32::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        Ok((buffer, Arguments::QueueDeleteOk(QueueDeleteOk { message_count })))
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct QueueUnbind {
    ticket: u16,
    queue_name: ShortStr,
    exchange_name: ShortStr,
    routing_key: ShortStr,
    args: FieldTable
}

impl Encode for QueueUnbind {
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.put_u16(self.ticket);
        self.queue_name.encode(buffer);
        self.exchange_name.encode(buffer);
        self.routing_key.encode(buffer);
        self.args.encode(buffer);
    }
}

impl Decode<Arguments> for QueueUnbind {
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        let (buffer, ticket) = match u16::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        let (buffer, queue_name) = match ShortStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        let (buffer, exchange_name) = match ShortStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        let (buffer, routing_key) = match ShortStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        let (buffer, args) = match FieldTable::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        Ok((buffer, Arguments::QueueUnbind(QueueUnbind { ticket, queue_name, exchange_name, routing_key, args })))
    }
}

pub struct QueueUnbindOk;

impl Encode for QueueUnbindOk {
    #[inline]
    fn encode(&self, _: &mut BytesMut) {
    }
}

impl Decode<Arguments> for QueueUnbindOk {
    #[inline]
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        Ok((buffer, Arguments::QueueUnbindOk(QueueUnbindOk)))
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct QueueProperties {
    flags: u32,
}

impl Encode for QueueProperties {
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.put_u32(self.flags);
    }
}

impl Decode<Property> for QueueProperties {
    fn decode(buffer: &[u8]) -> Result<(&[u8], Property), FrameDecodeErr>{
        let (buffer, flags) = match u32::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        Ok((buffer, Property::Queue(QueueProperties { flags })))
    }
}

pub enum QueueMethod {
    Declare,
    DeclareOk,
    Bind,
    BindOk,
    Unbind,
    UnbindOk,
    Purge,
    PurgeOk,
    Delete,
    DeleteOk,
    Unknown
}

impl MethodId for QueueMethod {
    fn method_id(&self) -> u16 {
        match self {
            QueueMethod::Declare => 10,
            QueueMethod::DeclareOk => 11,
            QueueMethod::Bind => 20,
            QueueMethod::BindOk => 21,
            QueueMethod::Unbind => 50,
            QueueMethod::UnbindOk => 51,
            QueueMethod::Purge => 30,
            QueueMethod::PurgeOk => 31,
            QueueMethod::Delete => 40,
            QueueMethod::DeleteOk => 41,
            QueueMethod::Unknown => 0xffff
        }
    }
}

impl Default for QueueMethod {
    fn default() -> Self {
        QueueMethod::Unknown
    }
}

impl From<u16> for QueueMethod {
    fn from(method_id: u16) -> Self {
        match method_id {
            10 => QueueMethod::Declare,
            11 => QueueMethod::DeclareOk,
            20 => QueueMethod::Bind,
            21 => QueueMethod::BindOk,
            50 => QueueMethod::Unbind,
            51 => QueueMethod::UnbindOk,
            30 => QueueMethod::Purge,
            31 => QueueMethod::PurgeOk,
            40 => QueueMethod::Delete,
            41 => QueueMethod::DeleteOk,
            _  => QueueMethod::Unknown
        }
    }
}