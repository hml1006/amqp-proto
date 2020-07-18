use property::Property;
use bytes::{BytesMut, BufMut};
use crate::common::{Encode, MethodId, Decode};
use crate::{ShortStr, FieldTable, Timestamp};
use crate::frame::{Arguments, Property};
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
            Err(e) => return Err(e)
        };
        let (buffer, prefetch_count) = match u16::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        let (buffer, flags) = match u8::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
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
            Err(e) => return Err(e)
        };
        let (buffer, queue_name) = match ShortStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        let (buffer, consumer_tag) = match ShortStr::decode(buffer) {
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
            Err(e) => return Err(e)
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
            Err(e) => return Err(e)
        };
        let (buffer, flags) = match u8::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
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
            Err(e) => return Err(e)
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
            Err(e) => return Err(e)
        };
        let (buffer, reply_text) = match ShortStr::decode(buffer) {
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
            Err(e) => return Err(e)
        };
        let (buffer, delivery_tag) = match u64::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        let (buffer, flags) = match u8::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        let redelivered = if flags & (1 << 0) != 0 { true } else { false };
        let (buffer, exchange_name) = match ShortStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        let (buffer, routing_key) = match ShortStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
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
            Err(e) => return Err(e)
        };
        let (buffer, flags) = match u8::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        let redelivered = if flags & (1 << 0) != 0 { true } else { false };
        let (buffer, exchange_name) = match ShortStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        let (buffer, routing_key) = match ShortStr::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        let (buffer, message_count) = match u32::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
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
            Err(e) => return Err(e)
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
            Err(e) => return Err(e)
        };
        let (buffer, flags) = match u8::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
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
            Err(e) => return Err(e)
        };
        let (buffer, flags) = match u8::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
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
            Err(e) => return Err(e)
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
            Err(e) => return Err(e)
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
            Err(e) => return Err(e)
        };
        let (buffer, flags) = match u8::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        let multiple = if flags & (1 << 0) != 0 { true } else { false };
        let requeue = if flags & (1 << 1) != 0 { true } else { false };
        Ok((buffer, Arguments::BasicNack(BasicNack {delivery_tag, multiple, requeue })))
    }
}

#[derive(Property, Default)]
#[property(get(public), set(disable))]
pub struct BasicProperties {
    flags: u32,
    content_type: ShortStr,
    content_encoding: ShortStr,
    headers: FieldTable,
    delivery_mode: u8,
    priority: u8,
    correlation_id: ShortStr,
    reply_to: ShortStr,
    expiration: ShortStr,
    message_id: ShortStr,
    timestamp: Timestamp,
    basic_type: ShortStr,
    user_id: ShortStr,
    app_id: ShortStr,
    cluster_id: ShortStr
}

impl BasicProperties {
    pub fn set_content_type(&mut self, content_type: ShortStr) {
        self.flags |= BasicProperties::CONTENT_TYPE_FLAG;
        self.content_type = content_type;
    }

    pub fn set_content_encoding(&mut self, content_encoding: ShortStr) {
        self.flags |= BasicProperties::CONTENT_ENCODING_FLAG;
        self.content_encoding = content_encoding;
    }

    pub fn set_headers(&mut self, headers: FieldTable) {
        self.flags |= BasicProperties::HEADERS_FLAG;
        self.headers = headers;
    }

    pub fn set_delivery_mode(&mut self, delivery_mode: u8) {
        self.flags |= BasicProperties::DELIVERY_FLAG;
        self.delivery_mode = delivery_mode;
    }

    pub fn set_priority(&mut self, priority: u8) {
        self.flags |= BasicProperties::PRIORITY_FLAG;
        self.priority = priority;
    }

    pub fn set_correlation_id(&mut self, correlation_id: ShortStr) {
        self.flags |= BasicProperties::CORRELATION_ID_FLAG;
        self.correlation_id = correlation_id;
    }

    pub fn set_reply_to(&mut self, reply_to: ShortStr) {
        self.flags |= BasicProperties::REPLY_TO_FLAG;
        self.reply_to = reply_to;
    }

    pub fn set_expiration(&mut self, expiration: ShortStr) {
        self.flags |= BasicProperties::EXPIRATION_FLAG;
        self.expiration = expiration;
    }

    pub fn set_message_id(&mut self, message_id: ShortStr) {
        self.flags |= BasicProperties::MESSAGE_ID_FLAG;
        self.message_id = message_id;
    }

    pub fn set_timestamp(&mut self, timestamp: Timestamp) {
        self.flags |= BasicProperties::TIMESTAMP_FLAG;
        self.timestamp = timestamp;
    }

    pub fn set_basic_type(&mut self, basic_type: ShortStr) {
        self.flags |= BasicProperties::BASIC_TYPE_FLAG;
        self.basic_type = basic_type;
    }

    pub fn set_user_id(&mut self, user_id: ShortStr) {
        self.flags |= BasicProperties::USER_ID_FLAG;
        self.user_id = user_id;
    }

    pub fn set_app_id(&mut self, app_id: ShortStr) {
        self.flags |= BasicProperties::APP_ID_FLAG;
        self.app_id = app_id;
    }

    pub fn set_cluster_id(&mut self, cluster_id: ShortStr) {
        self.flags |= BasicProperties::CLUSTER_ID_FLAG;
        self.cluster_id = cluster_id;
    }
}

impl Encode for BasicProperties {
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.put_u32(self.flags);
        if self.flags & BasicProperties::CONTENT_TYPE_FLAG != 0 {
            self.content_type.encode(buffer);
        }

        if self.flags & BasicProperties::CONTENT_ENCODING_FLAG != 0 {
            self.content_encoding.encode(buffer);
        }

        if self.flags & BasicProperties::HEADERS_FLAG != 0 {
            self.headers.encode(buffer);
        }

        if self.flags & BasicProperties::DELIVERY_FLAG != 0 {
            buffer.put_u8(self.delivery_mode);
        }

        if self.flags & BasicProperties::PRIORITY_FLAG != 0 {
            buffer.put_u8(self.priority);
        }

        if self.flags & BasicProperties::CORRELATION_ID_FLAG != 0 {
            self.correlation_id.encode(buffer);
        }

        if self.flags & BasicProperties::REPLY_TO_FLAG != 0 {
            self.reply_to.encode(buffer);
        }

        if self.flags & BasicProperties::EXPIRATION_FLAG != 0 {
            self.expiration.encode(buffer);
        }

        if self.flags & BasicProperties::MESSAGE_ID_FLAG != 0 {
            self.message_id.encode(buffer);
        }

        if self.flags & BasicProperties::TIMESTAMP_FLAG != 0 {
            buffer.put_u64(self.timestamp);
        }

        if self.flags & BasicProperties::BASIC_TYPE_FLAG != 0 {
            self.basic_type.encode(buffer);
        }

        if self.flags & BasicProperties::USER_ID_FLAG != 0 {
            self.user_id.encode(buffer);
        }

        if self.flags & BasicProperties::APP_ID_FLAG != 0 {
            self.app_id.encode(buffer);
        }

        if self.flags & BasicProperties::CLUSTER_ID_FLAG != 0 {
            self.cluster_id.encode(buffer);
        }
    }
}

impl Decode<Property> for BasicProperties {
    fn decode(buffer: &[u8]) -> Result<(&[u8], Property), FrameDecodeErr>{
        let (buffer, flags) = match u32::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(e)
        };
        let mut properties = BasicProperties::default();
        let buffer = if flags & BasicProperties::CONTENT_TYPE_FLAG != 0 {
            let (buffer, content_type) = match ShortStr::decode(buffer) {
                Ok(ret) => ret,
                Err(e) => return Err(e)
            };
            properties.set_content_type(content_type);
            buffer
        } else { buffer };

        let buffer = if flags & BasicProperties::CONTENT_ENCODING_FLAG != 0 {
            let (buffer, content_encoding) = match ShortStr::decode(buffer) {
                Ok(ret) => ret,
                Err(e) => return Err(e)
            };
            properties.set_content_encoding(content_encoding);
            buffer
        } else { buffer };

        let buffer = if flags & BasicProperties::HEADERS_FLAG != 0 {
            let (buffer, headers) = match FieldTable::decode(buffer) {
                Ok(ret) => ret,
                Err(e) => return Err(e)
            };
            properties.set_headers(headers);
            buffer
        } else { buffer };

        let buffer = if flags & BasicProperties::DELIVERY_FLAG != 0 {
            let (buffer, delivery_mode) = match u8::decode(buffer) {
                Ok(ret) => ret,
                Err(e) => return Err(e)
            };
            properties.set_delivery_mode(delivery_mode);
            buffer
        } else { buffer };

        let buffer = if flags & BasicProperties::PRIORITY_FLAG != 0 {
            let (buffer, priority) = match u8::decode(buffer) {
                Ok(ret) => ret,
                Err(e) => return Err(e)
            };
            properties.set_priority(priority);
            buffer
        } else { buffer };

        let buffer = if flags & BasicProperties::CORRELATION_ID_FLAG != 0 {
            let (buffer, correlation_id) = match ShortStr::decode(buffer) {
                Ok(ret) => ret,
                Err(e) => return Err(e)
            };
            properties.set_correlation_id(correlation_id);
            buffer
        } else { buffer };

        let buffer = if flags & BasicProperties::REPLY_TO_FLAG != 0 {
            let (buffer, reply_to) = match ShortStr::decode(buffer) {
                Ok(ret) => ret,
                Err(e) => return Err(e)
            };
            properties.set_reply_to(reply_to);
            buffer
        } else { buffer };

        let buffer = if flags & BasicProperties::EXPIRATION_FLAG != 0 {
            let (buffer, expiration) = match ShortStr::decode(buffer) {
                Ok(ret) => ret,
                Err(e) => return Err(e)
            };
            properties.set_expiration(expiration);
            buffer
        } else { buffer };

        let buffer = if flags & BasicProperties::MESSAGE_ID_FLAG != 0 {
            let (buffer, message_id) = match ShortStr::decode(buffer) {
                Ok(ret) => ret,
                Err(e) => return Err(e)
            };
            properties.set_message_id(message_id);
            buffer
        } else { buffer };

        let buffer = if flags & BasicProperties::TIMESTAMP_FLAG != 0 {
            let (buffer, timestamp) = match u64::decode(buffer) {
                Ok(ret) => ret,
                Err(e) => return Err(e)
            };
            properties.set_timestamp(timestamp);
            buffer
        } else { buffer };

        let buffer = if flags & BasicProperties::BASIC_TYPE_FLAG != 0 {
            let (buffer, basic_type) = match ShortStr::decode(buffer) {
                Ok(ret) => ret,
                Err(e) => return Err(e)
            };
            properties.set_basic_type(basic_type);
            buffer
        } else { buffer };

        let buffer = if flags & BasicProperties::USER_ID_FLAG != 0 {
            let (buffer, user_id) = match ShortStr::decode(buffer) {
                Ok(ret) => ret,
                Err(e) => return Err(e)
            };
            properties.set_user_id(user_id);
            buffer
        } else { buffer };

        let buffer = if flags & BasicProperties::APP_ID_FLAG != 0 {
            let (buffer, app_id) = match ShortStr::decode(buffer) {
                Ok(ret) => ret,
                Err(e) => return Err(e)
            };
            properties.set_app_id(app_id);
            buffer
        } else { buffer };

        let buffer = if flags & BasicProperties::CLUSTER_ID_FLAG != 0 {
            let (buffer, cluster_id) = match ShortStr::decode(buffer) {
                Ok(ret) => ret,
                Err(e) => return Err(e)
            };
            properties.set_cluster_id(cluster_id);
            buffer
        } else { buffer };
        Ok((buffer, Property::Basic(properties)))
    }
}

impl BasicProperties {
    const CONTENT_TYPE_FLAG: u32 = 1 << 15;
    const CONTENT_ENCODING_FLAG: u32 = 1 << 14;
    const HEADERS_FLAG: u32 = 1 << 13;
    const DELIVERY_FLAG: u32 = 1 << 12;
    const PRIORITY_FLAG: u32 = 1 << 11;
    const CORRELATION_ID_FLAG: u32 = 1 << 10;
    const REPLY_TO_FLAG: u32 = 1 << 9;
    const EXPIRATION_FLAG: u32 = 1 << 8;
    const MESSAGE_ID_FLAG: u32 = 1 << 7;
    const TIMESTAMP_FLAG: u32 = 1 << 6;
    const BASIC_TYPE_FLAG: u32 = 1 << 5;
    const USER_ID_FLAG: u32 = 1 << 4;
    const APP_ID_FLAG: u32 = 1 << 3;
    const CLUSTER_ID_FLAG: u32 = 1 << 2;
}


pub enum BasicMethod {
    Qos,
    QosOk,
    Consume,
    ConsumeOk,
    Cancel,
    CancelOk,
    Publish,
    Return,
    Deliver,
    Get,
    GetOk,
    GetEmpty,
    Ack,
    Reject,
    RecoverAsync,
    Recover,
    RecoverOk,
    Nack,
    Unknown
}

impl MethodId for BasicMethod {
    fn method_id(&self) -> u16 {
        match self {
            BasicMethod::Qos => 10,
            BasicMethod::QosOk => 11,
            BasicMethod::Consume => 20,
            BasicMethod::ConsumeOk => 21,
            BasicMethod::Cancel => 30,
            BasicMethod::CancelOk => 31,
            BasicMethod::Publish => 40,
            BasicMethod::Return => 50,
            BasicMethod::Deliver => 60,
            BasicMethod::Get => 70,
            BasicMethod::GetOk => 71,
            BasicMethod::GetEmpty => 72,
            BasicMethod::Ack => 80,
            BasicMethod::Reject => 90,
            BasicMethod::RecoverAsync => 100,
            BasicMethod::Recover => 110,
            BasicMethod::RecoverOk => 111,
            BasicMethod::Nack => 120,
            BasicMethod::Unknown => 0xffff
        }
    }
}

impl Default for BasicMethod {
    fn default() -> Self {
        BasicMethod::Unknown
    }
}

impl From<u16> for BasicMethod {
    fn from(method_id: u16) -> Self {
        match method_id {
            10 => BasicMethod::Qos,
            11 => BasicMethod::QosOk,
            20 => BasicMethod::Consume,
            21 => BasicMethod::ConsumeOk,
            30 => BasicMethod::Cancel,
            31 => BasicMethod::CancelOk,
            40 => BasicMethod::Publish,
            50 => BasicMethod::Return,
            60 => BasicMethod::Deliver,
            70 => BasicMethod::Get,
            71 => BasicMethod::GetOk,
            72 => BasicMethod::GetEmpty,
            80 => BasicMethod::Ack,
            90 => BasicMethod::Reject,
            100 => BasicMethod::RecoverAsync,
            110 => BasicMethod::Recover,
            111 => BasicMethod::RecoverOk,
            _  => BasicMethod::Unknown
        }
    }
}