use property::Property;
use bytes::{BytesMut, BufMut};
use crate::common::{WriteToBuf, MethodId};
use crate::{ShortStr, FieldTable, Timestamp};

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct BasicQos {
    prefetch_size: u32,
    prefetch_count: u16,
    global: bool
}

impl WriteToBuf for BasicQos {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u32(self.prefetch_size);
        buffer.put_u16(self.prefetch_count);
        buffer.put_u8(if self.global { 1 } else { 0 });
    }
}

pub struct BasicQosOk;

impl WriteToBuf for BasicQosOk {
    #[inline]
    fn write_to_buf(&self, _: &mut BytesMut) {
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

impl WriteToBuf for BasicConsume {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u16(self.ticket);
        self.queue_name.write_to_buf(buffer);
        self.consumer_tag.write_to_buf(buffer);
        let mut flag = 0u8;
        flag |= if self.no_local { 1 } else { 0 };
        flag |= if self.no_ack { 1 << 1 } else { 0 };
        flag |= if self.exclusive { 1 << 2 } else { 0 };
        flag |= if self.no_wait { 1 << 3 } else { 0 };
        buffer.put_u8(flag);
        self.args.write_to_buf(buffer);
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct BasicConsumeOk {
    consumer_tag: ShortStr
}

impl WriteToBuf for BasicConsumeOk {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        self.consumer_tag.write_to_buf(buffer);
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct BasicCancel {
    consumer_tag: ShortStr,
    no_wait: bool
}

impl WriteToBuf for BasicCancel {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        self.consumer_tag.write_to_buf(buffer);
        buffer.put_u8(if self.no_wait { 1 } else { 0 });
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct BasicCancelOk {
    consumer_tag: ShortStr
}

impl WriteToBuf for BasicCancelOk {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        self.consumer_tag.write_to_buf(buffer);
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

impl WriteToBuf for BasicPublish {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u16(self.ticket);
        self.exchange_name.write_to_buf(buffer);
        self.routing_key.write_to_buf(buffer);
        let mut flag = 0u8;
        flag |= if self.mandatory { 1 } else { 0 };
        flag |= if self.immediate { 1 << 1 } else { 0 };
        buffer.put_u8(flag);
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

impl WriteToBuf for BasicReturn {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u16(self.reply_code);
        self.reply_text.write_to_buf(buffer);
        self.exchange_name.write_to_buf(buffer);
        self.routing_key.write_to_buf(buffer);
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

impl WriteToBuf for BasicDeliver {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        self.consumer_tag.write_to_buf(buffer);
        buffer.put_u64(self.delivery_tag);
        buffer.put_u8(if self.redelivered { 1 } else { 0 });
        self.exchange_name.write_to_buf(buffer);
        self.routing_key.write_to_buf(buffer);
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct BasicGet {
    ticket: u16,
    queue_name: ShortStr,
    no_ack: bool
}

impl WriteToBuf for BasicGet {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u16(self.ticket);
        self.queue_name.write_to_buf(buffer);
        buffer.put_u8(if self.no_ack { 1 } else { 0 });
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

impl WriteToBuf for BasicGetOk {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u64(self.delivery_tag);
        buffer.put_u8(if self.redelivered { 1 } else { 0 });
        self.exchange_name.write_to_buf(buffer);
        self.routing_key.write_to_buf(buffer);
        buffer.put_u32(self.message_count);
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct BasicGetEmpty {
    cluster_id: ShortStr
}

impl WriteToBuf for BasicGetEmpty {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        self.cluster_id.write_to_buf(buffer);
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct BasicAck {
    delivery_tag: u64,
    multiple: bool
}

impl WriteToBuf for BasicAck {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u64(self.delivery_tag);
        buffer.put_u8(if self.multiple { 1 } else { 0 });
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct BasicReject {
    delivery_tag: u64,
    requeue: bool
}

impl WriteToBuf for BasicReject {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u64(self.delivery_tag);
        buffer.put_u8(if self.requeue { 1 } else { 0 });
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct BasicRecoverAsync {
    requeue: bool
}

impl WriteToBuf for BasicRecoverAsync {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u8(if self.requeue { 1 } else { 0 });
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct BasicRecover {
    requeue: bool
}

impl WriteToBuf for BasicRecover {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u8(if self.requeue { 1 } else { 0 });
    }
}

pub struct BasicRecoverOk;

impl WriteToBuf for BasicRecoverOk {
    #[inline]
    fn write_to_buf(&self, _: &mut BytesMut) {
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct BasicNack {
    delivery_tag: u64,
    multiple: bool,
    requeue: bool
}

impl WriteToBuf for BasicNack {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u64(self.delivery_tag);
        let mut flag = 0u8;
        flag |= if self.multiple { 1 } else { 0 };
        flag |= if self.requeue { 1 << 1 } else { 0 };
        buffer.put_u8(flag);
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

impl WriteToBuf for BasicProperties {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u32(self.flags);
        if self.flags & BasicProperties::CONTENT_TYPE_FLAG != 0 {
            self.content_type.write_to_buf(buffer);
        }

        if self.flags & BasicProperties::CONTENT_ENCODING_FLAG != 0 {
            self.content_encoding.write_to_buf(buffer);
        }

        if self.flags & BasicProperties::HEADERS_FLAG != 0 {
            self.headers.write_to_buf(buffer);
        }

        if self.flags & BasicProperties::DELIVERY_FLAG != 0 {
            buffer.put_u8(self.delivery_mode);
        }

        if self.flags & BasicProperties::PRIORITY_FLAG != 0 {
            buffer.put_u8(self.priority);
        }

        if self.flags & BasicProperties::CORRELATION_ID_FLAG != 0 {
            self.correlation_id.write_to_buf(buffer);
        }

        if self.flags & BasicProperties::REPLY_TO_FLAG != 0 {
            self.reply_to.write_to_buf(buffer);
        }

        if self.flags & BasicProperties::EXPIRATION_FLAG != 0 {
            self.expiration.write_to_buf(buffer);
        }

        if self.flags & BasicProperties::MESSAGE_ID_FLAG != 0 {
            self.message_id.write_to_buf(buffer);
        }

        if self.flags & BasicProperties::TIMESTAMP_FLAG != 0 {
            buffer.put_u64(self.timestamp);
        }

        if self.flags & BasicProperties::BASIC_TYPE_FLAG != 0 {
            self.basic_type.write_to_buf(buffer);
        }

        if self.flags & BasicProperties::USER_ID_FLAG != 0 {
            self.user_id.write_to_buf(buffer);
        }

        if self.flags & BasicProperties::APP_ID_FLAG != 0 {
            self.app_id.write_to_buf(buffer);
        }

        if self.flags & BasicProperties::CLUSTER_ID_FLAG != 0 {
            self.cluster_id.write_to_buf(buffer);
        }
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