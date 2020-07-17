use property::Property;
use bytes::{BytesMut, BufMut};
use crate::{ShortStr, FieldTable};
use crate::common::{WriteToBuf, MethodId};

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ExchangeDeclare {
    ticket: u16,
    exchange_name: ShortStr,
    exchange_type: ShortStr,
    passive: bool,
    durable: bool,
    auto_delete: bool,
    internal: bool,
    no_wait: bool,
    args: FieldTable
}

impl WriteToBuf for ExchangeDeclare {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u16(self.ticket);
        self.exchange_name.write_to_buf(buffer);
        self.exchange_type.write_to_buf(buffer);
        let mut flag = 0u8;
        flag |= if self.passive { 1 } else { 0 };
        flag |= if self.durable { 1 << 1 } else { 0 };
        flag |= if self.auto_delete { 1 << 2 } else { 0 };
        flag |= if self.internal { 1 << 3 } else { 0 };
        flag |= if self.no_wait { 1 << 4 } else { 0 };
        buffer.put_u8(flag);
        self.args.write_to_buf(buffer);
    }
}

pub struct ExchangeDeclareOk;

impl WriteToBuf for ExchangeDeclareOk {
    #[inline]
    fn write_to_buf(&self, _: &mut BytesMut) {
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ExchangeDelete {
    ticket: u16,
    exchange_name: ShortStr,
    if_unused: bool,
    no_wait: bool
}

impl WriteToBuf for ExchangeDelete {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u16(self.ticket);
        self.exchange_name.write_to_buf(buffer);
        let mut flag = 0u8;
        flag |= if self.if_unused { 1 } else { 0 };
        flag |= if self.no_wait { 1 << 1 } else { 0};
        buffer.put_u8(flag);
    }
}

pub struct ExchangeDeleteOk;

impl WriteToBuf for ExchangeDeleteOk {
    #[inline]
    fn write_to_buf(&self, _: &mut BytesMut) {
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ExchangeBind {
    ticket: u16,
    destination: ShortStr,
    source: ShortStr,
    routing_key: ShortStr,
    no_wait: bool,
    args: FieldTable
}

impl WriteToBuf for ExchangeBind {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u16(self.ticket);
        self.destination.write_to_buf(buffer);
        self.source.write_to_buf(buffer);
        self.routing_key.write_to_buf(buffer);
        buffer.put_u8(if self.no_wait { 1 } else { 0});
        self.args.write_to_buf(buffer);
    }
}

pub struct ExchangeBindOk;

impl WriteToBuf for ExchangeBindOk {
    #[inline]
    fn write_to_buf(&self, _: &mut BytesMut) {
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ExchangeUnbind {
    ticket: u16,
    destination: ShortStr,
    source: ShortStr,
    routing_key: ShortStr,
    no_wait: bool,
    args: FieldTable
}

impl WriteToBuf for ExchangeUnbind {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u16(self.ticket);
        self.destination.write_to_buf(buffer);
        self.source.write_to_buf(buffer);
        self.routing_key.write_to_buf(buffer);
        buffer.put_u8(if self.no_wait { 1 } else { 0});
        self.args.write_to_buf(buffer);
    }
}

pub struct ExchangeUnbindOk;

impl WriteToBuf for ExchangeUnbindOk {
    #[inline]
    fn write_to_buf(&self, _: &mut BytesMut) {
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ExchangeProperties {
    flags: u32,
}

impl WriteToBuf for ExchangeProperties {
    #[inline]
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u32(self.flags);
    }
}


pub enum ExchangeMethod {
    Declare,
    DeclareOk,
    Delete,
    DeleteOk,
    Bind,
    BindOk,
    Unbind,
    UnbindOk,
    Unknown
}

impl MethodId for ExchangeMethod {
    fn method_id(&self) -> u16 {
        match self {
            ExchangeMethod::Declare => 10,
            ExchangeMethod::DeclareOk => 11,
            ExchangeMethod::Delete => 20,
            ExchangeMethod::DeleteOk => 21,
            ExchangeMethod::Bind => 30,
            ExchangeMethod::BindOk => 31,
            ExchangeMethod::Unbind => 40,
            ExchangeMethod::UnbindOk => 51,
            ExchangeMethod::Unknown => 0xffff
        }
    }
}

impl Default for ExchangeMethod {
    fn default() -> Self {
        ExchangeMethod::Unknown
    }
}

impl From<u16> for ExchangeMethod {
    fn from(method_id: u16) -> Self {
        match method_id {
            10 => ExchangeMethod::Declare,
            11 => ExchangeMethod::DeclareOk,
            20 => ExchangeMethod::Delete,
            21 => ExchangeMethod::DeleteOk,
            30 => ExchangeMethod::Bind,
            31 => ExchangeMethod::BindOk,
            40 => ExchangeMethod::Unbind,
            51 => ExchangeMethod::UnbindOk,
            _  => ExchangeMethod::Unknown
        }
    }
}