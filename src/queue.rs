use property::Property;
use bytes::{BytesMut, BufMut};
use crate::{ShortStr, FieldTable};
use crate::common::{WriteToBuf, MethodId};

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

impl WriteToBuf for QueueDeclare {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u16(self.ticket);
        self.queue_name.write_to_buf(buffer);
        let mut flag = 0u8;
        flag |= if self.passive { 1 } else { 0 };
        flag |= if self.durable { 1 << 1 } else { 0};
        flag |= if self.exclusive { 1 << 2 } else { 0 };
        flag |= if self.auto_delete { 1 << 3 } else { 0 };
        flag |= if self.no_wait { 1 << 4 } else { 0 };
        buffer.put_u8(flag);
        self.args.write_to_buf(buffer);
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct QueueDeclareOk {
    queue_name: ShortStr,
    message_count: u32,
    consumer_count: u32
}

impl WriteToBuf for QueueDeclareOk {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        self.queue_name.write_to_buf(buffer);
        buffer.put_u32(self.message_count);
        buffer.put_u32(self.consumer_count);
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

impl WriteToBuf for QueueBind {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u16(self.ticket);
        self.queue_name.write_to_buf(buffer);
        self.exchange_name.write_to_buf(buffer);
        self.routing_key.write_to_buf(buffer);
        buffer.put_u8(if self.no_wait { 1 } else { 0 });
        self.args.write_to_buf(buffer);
    }
}

pub struct QueueBindOk;

impl WriteToBuf for QueueBindOk {
    #[inline]
    fn write_to_buf(&self, _: &mut BytesMut) {
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct QueuePurge {
    ticket: u16,
    queue_name: ShortStr,
    no_wait: bool
}

impl WriteToBuf for QueuePurge {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u16(self.ticket);
        self.queue_name.write_to_buf(buffer);
        buffer.put_u8(if self.no_wait { 1 } else { 0 });
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct QueuePurgeOk {
    message_count: u32
}

impl WriteToBuf for QueuePurgeOk {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u32(self.message_count);
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

impl WriteToBuf for QueueDelete {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u16(self.ticket);
        self.queue_name.write_to_buf(buffer);
        let mut flag = 0u8;
        flag |= if self.if_unused { 1 } else { 0};
        flag |= if self.if_empty { 1 << 1 } else { 0 };
        flag |= if self.no_wait { 1 << 2 } else { 0 };
        buffer.put_u8(flag);
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct QueueDeleteOk {
    message_count: u32
}

impl WriteToBuf for QueueDeleteOk {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u32(self.message_count);
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

impl WriteToBuf for QueueUnbind {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u16(self.ticket);
        self.queue_name.write_to_buf(buffer);
        self.exchange_name.write_to_buf(buffer);
        self.routing_key.write_to_buf(buffer);
        self.args.write_to_buf(buffer);
    }
}

pub struct QueueUnbindOk;

impl WriteToBuf for QueueUnbindOk {
    #[inline]
    fn write_to_buf(&self, _: &mut BytesMut) {
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct QueueProperties {
    flags: u32,
}

impl WriteToBuf for QueueProperties {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u32(self.flags);
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