use property::Property;
use bytes::{BytesMut, BufMut};
use crate::{ShortStr, FieldTable};
use crate::frame::base::{Encode, Arguments, Decode};
use crate::error::FrameDecodeErr;

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct BasicQos {
    prefetch_size: u32,
    prefetch_count: u16,
    global: bool
}

impl Encode for BasicQos {
    #[inline]
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.put_u32(self.prefetch_size);
        buffer.put_u16(self.prefetch_count);
        buffer.put_u8(if self.global { 1 } else { 0 });
    }
}

impl Decode<Arguments> for BasicQos {
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr> {
        let (buffer, prefetch_size) = match u32::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicQos prefetch_size -> {}", e)))
        };
        let (buffer, prefetch_count) = match u16::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicQos prefetch_count -> {}", e)))
        };
        let (buffer, flags) = match u8::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicQos flags -> {}", e)))
        };
        let global = if 0 != (flags & (1 << 0)) {
            true
        } else {
            false
        };
        Ok((buffer, Arguments::BasicQos(BasicQos { prefetch_size, prefetch_count, global})))
    }
}

pub struct BasicQosOk;

impl Encode for BasicQosOk {
    #[inline]
    fn encode(&self, _: &mut BytesMut) {
    }
}

impl Decode<Arguments> for BasicQosOk {
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        Ok((buffer, Arguments::BasicQosOk(BasicQosOk)))
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct BasicConsume {
    ticket: u16,
    queue_name: ShortStr,
    consumer_tag: ShortStr,
    no_local: bool,
    no_ack: bool,
    exclusive: bool,
    no_wait: bool,
    args: FieldTable
}

impl Encode for BasicConsume {
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.put_u16(self.ticket);
        self.queue_name.encode(buffer);
        self.consumer_tag.encode(buffer);
        let mut flag = 0u8;
        flag |= if self.no_local { 1 } else { 0 };
        flag |= if self.no_ack { 1 << 1 } else { 0 };
        flag |= if self.exclusive { 1 << 2 } else { 0 };
        flag |= if self.no_wait { 1 << 3 } else { 0 };
        buffer.put_u8(flag);
        self.args.encode(buffer);
    }
}

impl Decode<Arguments> for BasicConsume {
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr> {
        let (buffer, ticket) = match u16::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicConsume ticket -> {}", e)))
        };
        let (buffer, queue_name) = match ShortStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicConsume queue_name -> {}", e)))
        };
        let (buffer, consumer_tag) = match ShortStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicConsume consumer_tag -> {}", e)))
        };
        let (buffer, flags) = match u8::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicConsume flags -> {}", e)))
        };
        let (buffer, args) = match FieldTable::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicDeliver consumer_tag -> {}", e)))
        };
        let no_local = if flags & (1 << 0) != 0 { true } else { false };
        let no_ack = if flags & (1 << 1) != 0 { true } else { false };
        let exclusive = if flags & (1 << 2) != 0 { true } else { false };
        let no_wait = if flags & (1 << 3) != 0 { true } else { false };
        Ok((buffer, Arguments::BasicConsume(BasicConsume { ticket, queue_name, consumer_tag, no_local, no_ack, exclusive, no_wait, args})))
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct BasicConsumeOk {
    consumer_tag: ShortStr
}

impl Encode for BasicConsumeOk {
    fn encode(&self, buffer: &mut BytesMut) {
        self.consumer_tag.encode(buffer);
    }
}

impl Decode<Arguments> for BasicConsumeOk {
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        let (buffer, consumer_tag) = match ShortStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicConsumeOk consumer_tag -> {}", e)))
        };
        Ok((buffer, Arguments::BasicConsumeOk(BasicConsumeOk { consumer_tag})))
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct BasicCancel {
    consumer_tag: ShortStr,
    no_wait: bool
}

impl Encode for BasicCancel {
    fn encode(&self, buffer: &mut BytesMut) {
        self.consumer_tag.encode(buffer);
        buffer.put_u8(if self.no_wait { 1 } else { 0 });
    }
}

impl Decode<Arguments> for BasicCancel {
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        let (buffer, consumer_tag) = match ShortStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicCancel consumer_tag -> {}", e)))
        };
        let (buffer, flags) = match u8::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicCancel flags -> {}", e)))
        };
        let no_wait = if flags & (1 << 0) != 0 { true } else { false };
        Ok((buffer, Arguments::BasicCancel(BasicCancel { consumer_tag, no_wait })))
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct BasicCancelOk {
    consumer_tag: ShortStr
}

impl Encode for BasicCancelOk {
    fn encode(&self, buffer: &mut BytesMut) {
        self.consumer_tag.encode(buffer);
    }
}

impl Decode<Arguments> for BasicCancelOk {
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        let (buffer, consumer_tag) = match ShortStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicCancelOk consumer_tag -> {}", e)))
        };
        Ok((buffer, Arguments::BasicCancelOk(BasicCancelOk { consumer_tag })))
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct BasicPublish {
    ticket: u16,
    exchange_name: ShortStr,
    routing_key: ShortStr,
    mandatory: bool,
    immediate: bool
}

impl Encode for BasicPublish {
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.put_u16(self.ticket);
        self.exchange_name.encode(buffer);
        self.routing_key.encode(buffer);
        let mut flag = 0u8;
        flag |= if self.mandatory { 1 } else { 0 };
        flag |= if self.immediate { 1 << 1 } else { 0 };
        buffer.put_u8(flag);
    }
}

impl Decode<Arguments> for BasicPublish {
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        let (buffer, ticket) = match u16::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicPublish ticket -> {}", e)))
        };
        let (buffer, exchange_name) = match ShortStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicPublish exchange_name -> {}", e)))
        };
        let (buffer, routing_key) = match ShortStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicPublish routing_key -> {}", e)))
        };
        let (buffer, flags) = match u8::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicPublish flags -> {}", e)))
        };
        let mandatory = if flags & (1 << 0) != 0 { true } else { false };
        let immediate = if flags & (1 << 1) != 0 { true } else { false };
        Ok((buffer, Arguments::BasicPublish(BasicPublish { ticket, exchange_name, routing_key, mandatory, immediate })))
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct BasicReturn {
    reply_code: u16,
    reply_text: ShortStr,
    exchange_name: ShortStr,
    routing_key: ShortStr
}

impl Encode for BasicReturn {
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.put_u16(self.reply_code);
        self.reply_text.encode(buffer);
        self.exchange_name.encode(buffer);
        self.routing_key.encode(buffer);
    }
}

impl Decode<Arguments> for BasicReturn {
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        let (buffer, reply_code) = match u16::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicReturn reply_code -> {}", e)))
        };
        let (buffer, reply_text) = match ShortStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicReturn reply_text -> {}", e)))
        };
        let (buffer, exchange_name) = match ShortStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicReturn exchange_name -> {}", e)))
        };
        let (buffer, routing_key) = match ShortStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicReturn routing_key -> {}", e)))
        };
        Ok((buffer, Arguments::BasicReturn(BasicReturn { reply_code, reply_text, exchange_name, routing_key })))
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct BasicDeliver {
    consumer_tag: ShortStr,
    delivery_tag: u64,
    redelivered: bool,
    exchange_name: ShortStr,
    routing_key: ShortStr
}

impl Encode for BasicDeliver {
    fn encode(&self, buffer: &mut BytesMut) {
        self.consumer_tag.encode(buffer);
        buffer.put_u64(self.delivery_tag);
        buffer.put_u8(if self.redelivered { 1 } else { 0 });
        self.exchange_name.encode(buffer);
        self.routing_key.encode(buffer);
    }
}

impl Decode<Arguments> for BasicDeliver {
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        let (buffer, consumer_tag) = match ShortStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicDeliver consumer_tag -> {}", e)))
        };
        let (buffer, delivery_tag) = match u64::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicDeliver delivery_tag -> {}", e)))
        };
        let (buffer, flags) = match u8::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicDeliver flags -> {}", e)))
        };
        let redelivered = if flags & (1 << 0) != 0 { true } else { false };
        let (buffer, exchange_name) = match ShortStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicDeliver exchange_name -> {}", e)))
        };
        let (buffer, routing_key) = match ShortStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicDeliver routing_key -> {}", e)))
        };
        Ok((buffer, Arguments::BasicDeliver(BasicDeliver { consumer_tag, delivery_tag, redelivered, exchange_name, routing_key})))
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct BasicGet {
    ticket: u16,
    queue_name: ShortStr,
    no_ack: bool
}

impl Encode for BasicGet {
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.put_u16(self.ticket);
        self.queue_name.encode(buffer);
        buffer.put_u8(if self.no_ack { 1 } else { 0 });
    }
}

impl Decode<Arguments> for BasicGet {
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        let (buffer, ticket) = match u16::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicGet ticket -> {}", e)))
        };
        let (buffer, queue_name) = match ShortStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicGet queue_name -> {}", e)))
        };
        let (buffer, flags) = match u8::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicGet flags -> {}", e)))
        };
        let no_ack = if flags & (1 << 0) != 0 { true } else { false };
        Ok((buffer, Arguments::BasicGet(BasicGet { ticket, queue_name, no_ack })))
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct BasicGetOk {
    delivery_tag: u64,
    redelivered: bool,
    exchange_name: ShortStr,
    routing_key: ShortStr,
    message_count: u32
}

impl Encode for BasicGetOk {
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.put_u64(self.delivery_tag);
        buffer.put_u8(if self.redelivered { 1 } else { 0 });
        self.exchange_name.encode(buffer);
        self.routing_key.encode(buffer);
        buffer.put_u32(self.message_count);
    }
}

impl Decode<Arguments> for BasicGetOk {
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        let (buffer, delivery_tag) = match u64::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicGetOk delivery_tag -> {}", e)))
        };
        let (buffer, flags) = match u8::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicGetOk flags -> {}", e)))
        };
        let redelivered = if flags & (1 << 0) != 0 { true } else { false };
        let (buffer, exchange_name) = match ShortStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicGetOk exchange_name -> {}", e)))
        };
        let (buffer, routing_key) = match ShortStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicGetOk routing_key -> {}", e)))
        };
        let (buffer, message_count) = match u32::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicGetOk message_count -> {}", e)))
        };
        Ok((buffer, Arguments::BasicGetOk(BasicGetOk {delivery_tag, redelivered, exchange_name, routing_key, message_count})))
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct BasicGetEmpty {
    cluster_id: ShortStr
}

impl Encode for BasicGetEmpty {
    fn encode(&self, buffer: &mut BytesMut) {
        self.cluster_id.encode(buffer);
    }
}

impl Decode<Arguments> for BasicGetEmpty {
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        let (buffer, cluster_id) = match ShortStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicGetEmpty cluster_id -> {}", e)))
        };
        Ok((buffer, Arguments::BasicGetEmpty(BasicGetEmpty { cluster_id })))
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct BasicAck {
    delivery_tag: u64,
    multiple: bool
}

impl Encode for BasicAck {
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.put_u64(self.delivery_tag);
        buffer.put_u8(if self.multiple { 1 } else { 0 });
    }
}

impl Decode<Arguments> for BasicAck {
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        let (buffer, delivery_tag) = match u64::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicAck delivery_tag -> {}", e)))
        };
        let (buffer, flags) = match u8::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicAck flags -> {}", e)))
        };
        let multiple = if flags & (1 << 0) != 0 { true } else { false };
        Ok((buffer, Arguments::BasicAck(BasicAck { delivery_tag, multiple })))
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct BasicReject {
    delivery_tag: u64,
    requeue: bool
}

impl Encode for BasicReject {
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.put_u64(self.delivery_tag);
        buffer.put_u8(if self.requeue { 1 } else { 0 });
    }
}

impl Decode<Arguments> for BasicReject {
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        let (buffer, delivery_tag) = match u64::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicReject delivery_tag -> {}", e)))
        };
        let (buffer, flags) = match u8::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicReject flags -> {}", e)))
        };
        let requeue = if flags & (1 << 0) != 0 { true } else { false };
        Ok((buffer, Arguments::BasicReject(BasicReject { delivery_tag, requeue })))
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct BasicRecoverAsync {
    requeue: bool
}

impl Encode for BasicRecoverAsync {
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.put_u8(if self.requeue { 1 } else { 0 });
    }
}

impl Decode<Arguments> for BasicRecoverAsync {
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        let (buffer, flags) = match u8::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicRecoverAsync flags -> {}", e)))
        };
        let requeue = if flags & (1 << 0) != 0 { true } else { false };
        Ok((buffer, Arguments::BasicRecoverAsync(BasicRecoverAsync { requeue })))
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct BasicRecover {
    requeue: bool
}

impl Encode for BasicRecover {
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.put_u8(if self.requeue { 1 } else { 0 });
    }
}

impl Decode<Arguments> for BasicRecover {
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        let (buffer, flags) = match u8::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicRecover flags -> {}", e)))
        };
        let requeue = if flags & (1 << 0) != 0 { true } else { false };
        Ok((buffer, Arguments::BasicRecover(BasicRecover { requeue })))
    }
}

pub struct BasicRecoverOk;

impl Encode for BasicRecoverOk {
    #[inline]
    fn encode(&self, _: &mut BytesMut) {
    }
}

impl Decode<Arguments> for BasicRecoverOk {
    #[inline]
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        Ok((buffer, Arguments::BasicRecoverOk(BasicRecoverOk)))
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct BasicNack {
    delivery_tag: u64,
    multiple: bool,
    requeue: bool
}

impl Encode for BasicNack {
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.put_u64(self.delivery_tag);
        let mut flag = 0u8;
        flag |= if self.multiple { 1 } else { 0 };
        flag |= if self.requeue { 1 << 1 } else { 0 };
        buffer.put_u8(flag);
    }
}

impl Decode<Arguments> for BasicNack {
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        let (buffer, delivery_tag) = match u64::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicNack delivery_tag -> {}", e)))
        };
        let (buffer, flags) = match u8::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicNack flags -> {}", e)))
        };
        let multiple = if flags & (1 << 0) != 0 { true } else { false };
        let requeue = if flags & (1 << 1) != 0 { true } else { false };
        Ok((buffer, Arguments::BasicNack(BasicNack {delivery_tag, multiple, requeue })))
    }
}

