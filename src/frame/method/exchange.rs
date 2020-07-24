use property::Property;
use bytes::{BytesMut, BufMut};
use crate::{ShortStr, FieldTable};
use crate::frame::base::{Arguments, Decode, Encode};
use crate::error::FrameDecodeErr;

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

impl Encode for ExchangeDeclare {
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.put_u16(self.ticket);
        self.exchange_name.encode(buffer);
        self.exchange_type.encode(buffer);
        let mut flag = 0u8;
        flag |= if self.passive { 1 } else { 0 };
        flag |= if self.durable { 1 << 1 } else { 0 };
        flag |= if self.auto_delete { 1 << 2 } else { 0 };
        flag |= if self.internal { 1 << 3 } else { 0 };
        flag |= if self.no_wait { 1 << 4 } else { 0 };
        buffer.put_u8(flag);
        self.args.encode(buffer);
    }
}

impl Decode<Arguments> for ExchangeDeclare {
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        let (buffer, ticket) = match u16::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode ExchangeDeclare ticket -> {}", e)))
        };
        let (buffer, exchange_name) = match ShortStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode ExchangeDeclare exchange_name -> {}", e)))
        };
        let (buffer, exchange_type) = match ShortStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode ExchangeDeclare exchange_type -> {}", e)))
        };
        let (buffer, flags) = match u8::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode ExchangeDeclare flags -> {}", e)))
        };
        let passive = if flags & (1 << 0) != 0 { true } else { false };
        let durable = if flags & (1 << 1) != 0 { true } else { false };
        let auto_delete = if flags & (1 << 2) != 0 { true } else { false };
        let internal = if flags & (1 << 3) != 0 { true } else { false };
        let no_wait = if flags & (1 << 4) != 0 { true } else { false };
        let (buffer, args) = match FieldTable::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode ExchangeDeclare args -> {}", e)))
        };
        Ok((buffer, Arguments::ExchangeDeclare(ExchangeDeclare { ticket, exchange_name, exchange_type, passive, durable, auto_delete, internal, no_wait, args})))
    }
}

pub struct ExchangeDeclareOk;

impl Encode for ExchangeDeclareOk {
    #[inline]
    fn encode(&self, _: &mut BytesMut) {
    }
}

impl Decode<Arguments> for ExchangeDeclareOk {
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        Ok((buffer, Arguments::ExchangeDeclareOk(ExchangeDeclareOk)))
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

impl Encode for ExchangeDelete {
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.put_u16(self.ticket);
        self.exchange_name.encode(buffer);
        let mut flag = 0u8;
        flag |= if self.if_unused { 1 } else { 0 };
        flag |= if self.no_wait { 1 << 1 } else { 0};
        buffer.put_u8(flag);
    }
}

impl Decode<Arguments> for ExchangeDelete {
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        let (buffer, ticket) = match u16::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode ExchangeDelete ticket -> {}", e)))
        };
        let (buffer, exchange_name) = match ShortStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode ExchangeDelete exchange_name -> {}", e)))
        };
        let (buffer, flags) = match u8::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode ExchangeDelete flags -> {}", e)))
        };
        let if_unused = if flags & (1 << 0) != 0 { true } else { false };
        let no_wait = if flags & (1 << 1) != 0 { true } else { false };
        Ok((buffer, Arguments::ExchangeDelete(ExchangeDelete { ticket, exchange_name, if_unused, no_wait })))
    }
}

pub struct ExchangeDeleteOk;

impl Encode for ExchangeDeleteOk {
    #[inline]
    fn encode(&self, _: &mut BytesMut) {
    }
}

impl Decode<Arguments> for ExchangeDeleteOk {
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        Ok((buffer, Arguments::ExchangeDeleteOk(ExchangeDeleteOk)))
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

impl Encode for ExchangeBind {
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.put_u16(self.ticket);
        self.destination.encode(buffer);
        self.source.encode(buffer);
        self.routing_key.encode(buffer);
        buffer.put_u8(if self.no_wait { 1 } else { 0});
        self.args.encode(buffer);
    }
}

impl Decode<Arguments> for ExchangeBind {
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        let (buffer, ticket) = match u16::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode ExchangeBind ticket -> {}", e)))
        };
        let (buffer, destination) = match ShortStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode ExchangeBind destination -> {}", e)))
        };
        let (buffer, source) = match ShortStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode ExchangeBind source -> {}", e)))
        };
        let (buffer, routing_key) = match ShortStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode ExchangeBind routing_key -> {}", e)))
        };
        let (buffer, flags) = match u8::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode ExchangeBind flags -> {}", e)))
        };
        let no_wait = if flags & (1 << 0) != 0 { true } else { false };
        let (buffer, args) = match FieldTable::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode ExchangeBind args -> {}", e)))
        };
        Ok((buffer, Arguments::ExchangeBind(ExchangeBind { ticket, destination, source, routing_key, no_wait, args })))
    }
}

pub struct ExchangeBindOk;

impl Encode for ExchangeBindOk {
    #[inline]
    fn encode(&self, _: &mut BytesMut) {
    }
}

impl Decode<Arguments> for ExchangeBindOk {
    #[inline]
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        Ok((buffer, Arguments::ExchangeBindOk(ExchangeBindOk)))
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

impl Encode for ExchangeUnbind {
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.put_u16(self.ticket);
        self.destination.encode(buffer);
        self.source.encode(buffer);
        self.routing_key.encode(buffer);
        buffer.put_u8(if self.no_wait { 1 } else { 0});
        self.args.encode(buffer);
    }
}

impl Decode<Arguments> for ExchangeUnbind {
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        let (buffer, ticket) = match u16::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode ExchangeUnbind ticket -> {}", e)))
        };
        let (buffer, destination) = match ShortStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode ExchangeUnbind destination -> {}", e)))
        };
        let (buffer, source) = match ShortStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode ExchangeUnbind source -> {}", e)))
        };
        let (buffer, routing_key) = match ShortStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode ExchangeUnbind routing_key -> {}", e)))
        };
        let (buffer, flags) = match u8::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode ExchangeUnbind flags -> {}", e)))
        };
        let no_wait = if flags & (1 << 0) != 0 { true } else { false };
        let (buffer, args) = match FieldTable::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode ExchangeUnbind args -> {}", e)))
        };
        Ok((buffer, Arguments::ExchangeUnbind(ExchangeUnbind { ticket, destination, source, routing_key, no_wait, args })))
    }
}

pub struct ExchangeUnbindOk;

impl Encode for ExchangeUnbindOk {
    #[inline]
    fn encode(&self, _: &mut BytesMut) {
    }
}

impl Decode<Arguments> for ExchangeUnbindOk {
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        Ok((buffer, Arguments::ExchangeUnbindOk(ExchangeUnbindOk)))
    }
}
