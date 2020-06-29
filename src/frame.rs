use crate::{FieldTable, LongStr, Timestamp, ShortStr};
use property::Property;
use crate::WriteToBuf;
use bytes::{BytesMut, BufMut};

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ConnectionStart {
    version_major: u8,
    version_minor: u8,
    server_properties: FieldTable,
    mechanisms: LongStr,
    locales: LongStr,
}

impl WriteToBuf for ConnectionStart {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u8(self.version_major);
        buffer.put_u8(self.version_minor);
        self.server_properties.write_to_buf(buffer);
        self.mechanisms.write_to_buf(buffer);
        self.locales.write_to_buf(buffer);
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ConnectionStartOk {
    client_properties: FieldTable,
    mechanism: ShortStr,
    response: LongStr,
    locale: ShortStr
}

impl WriteToBuf for ConnectionStartOk {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        self.client_properties.write_to_buf(buffer);
        self.mechanism.write_to_buf(buffer);
        self.response.write_to_buf(buffer);
        self.locale.write_to_buf(buffer);
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ConnectionSecure {
    challenge: LongStr
}

impl WriteToBuf for ConnectionSecure {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        self.challenge.write_to_buf(buffer);
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ConnectionSecureOk {
    response: LongStr
}

impl WriteToBuf for ConnectionSecureOk {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        self.response.write_to_buf(buffer);
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ConnectionTune {
    channel_max: u16,
    frame_max: u32,
    heartbeat: u16
}

impl WriteToBuf for ConnectionTune {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u16(self.channel_max);
        buffer.put_u32(self.frame_max);
        buffer.put_u16(self.heartbeat);
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ConnectionTuneOk {
    channel_max: u16,
    frame_max: u32,
    heartbeat: u16
}

impl WriteToBuf for ConnectionTuneOk {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u16(self.channel_max);
        buffer.put_u32(self.frame_max);
        buffer.put_u16(self.heartbeat);
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ConnectionOpen {
    vhost: ShortStr,
    capabilities: ShortStr,  // rabbitmq used
    insist: bool
}

impl WriteToBuf for ConnectionOpen {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        self.vhost.write_to_buf(buffer);
        self.capabilities.write_to_buf(buffer);
        buffer.put_u8(if self.insist {1u8} else {0u8});
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ConnectionOpenOk {
    known_hosts: ShortStr
}

impl WriteToBuf for ConnectionOpenOk {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        self.known_hosts.write_to_buf(buffer);
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ConnectionClose {
    reply_code: u16,
    reply_text: ShortStr,
    class: Class,
    method: Method
}

impl WriteToBuf for ConnectionClose {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u16(self.reply_code);
        self.reply_text.write_to_buf(buffer);
        buffer.put_u16(self.class.class_id());
        buffer.put_u16(self.method.method_id())
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ConnectionCloseOk {
    dummy: u8               // fill struct
}

impl WriteToBuf for ConnectionCloseOk {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u8(self.dummy);
    }
}

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

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ChannelCloseOk {
    dummy: u8           // fill struct
}

impl WriteToBuf for ChannelCloseOk {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u8(self.dummy);
    }
}

// Accesss is deprecated in amqp0-9-1, this is just for compatibility
#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct AccessRequest {
    realm: ShortStr,
    exclusive: bool,
    passive: bool,
    active: bool,
    write: bool,
    read: bool
}

impl WriteToBuf for AccessRequest {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        self.realm.write_to_buf(buffer);
        // just fill 0
        buffer.put_u8(0);
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct AccessRequestOk {
    ticket: u16
}

impl WriteToBuf for AccessRequestOk {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u16(self.ticket);
    }
}

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

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ExchangeDeclareOk {
    dummy: u8           // fill struct
}

impl WriteToBuf for ExchangeDeclareOk {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u8(self.dummy);
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

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ExchangeDeleteOk {
    dummy: u8           // fill struct
}

impl WriteToBuf for ExchangeDeleteOk {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u8(self.dummy);
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

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ExchangeBindOk {
    dummy: u8
}

impl WriteToBuf for ExchangeBindOk {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u8(self.dummy);
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

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ExchangeUnbindOk {
    dummy: u8
}

impl WriteToBuf for ExchangeUnbindOk {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u8(self.dummy);
    }
}

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

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct QueueBindOk {
    dummy: u8           // fill struct
}

impl WriteToBuf for QueueBindOk {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u8(self.dummy);
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

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct QueueUnbindOk {
    dummy: u8           // fill struct
}

impl WriteToBuf for QueueUnbindOk {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u8(self.dummy);
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct BasicQos {
    prefetch_size: u32,
    prefetch_count: u32,
    global: bool
}

impl WriteToBuf for BasicQos {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u32(self.prefetch_size);
        buffer.put_u32(self.prefetch_count);
        buffer.put_u8(if self.global { 1 } else { 0 });
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct BasicQosOk {
    dummy: u8           // fill struct
}

impl WriteToBuf for BasicQosOk {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u8(self.dummy);
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

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct BasicRecoverOk {
    dummy: u8           // fill struct
}

impl WriteToBuf for BasicRecoverOk {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u8(self.dummy);
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
#[property(get(public), set(public))]
pub struct TxSelect {
    dummy: u8           // fill struct
}

impl WriteToBuf for TxSelect {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u8(self.dummy);
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct TxSelectOk {
    dummy: u8           // fill struct
}

impl WriteToBuf for TxSelectOk {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u8(self.dummy);
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct TxCommit {
    dummy: u8           // fill struct
}

impl WriteToBuf for TxCommit {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u8(self.dummy);
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct TxCommitOk {
    dummy: u8           // fill struct
}

impl WriteToBuf for TxCommitOk {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u8(self.dummy);
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct TxRollback {
    dummy: u8           // fill struct
}

impl WriteToBuf for TxRollback {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u8(self.dummy);
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct TxRollbackOk {
    dummy: u8           // fill struct
}

impl WriteToBuf for TxRollbackOk {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u8(self.dummy);
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct TxConfirmSelect {
    no_wait: bool
}

impl WriteToBuf for TxConfirmSelect {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u8(if self.no_wait { 1 } else { 0 });
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct TxConfirmSelectOk {
    dummy: u8           // fill struct
}

impl WriteToBuf for TxConfirmSelectOk {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u8(self.dummy);
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ConnectionProperties {
    flags: u32,
    dummy: u8           // fill struct
}

impl WriteToBuf for ConnectionProperties {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u32(self.flags);
        buffer.put_u8(self.dummy);
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ChannelProperties {
    flags: u32,
    dummy: u8           // fill struct
}

impl WriteToBuf for ChannelProperties {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u32(self.flags);
        buffer.put_u8(self.dummy);
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct AccessProperties {
    flags: u32,
    dummy: u8           // fill struct
}

impl WriteToBuf for AccessProperties {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u32(self.flags);
        buffer.put_u8(self.dummy);
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ExchangeProperties {
    flags: u32,
    dummy: u8           // fill struct
}

impl WriteToBuf for ExchangeProperties {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u32(self.flags);
        buffer.put_u8(self.dummy);
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct QueueProperties {
    flags: u32,
    dummy: u8           // fill struct
}

impl WriteToBuf for QueueProperties {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u32(self.flags);
        buffer.put_u8(self.dummy);
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

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct TxProperties {
    flags: u32,
    dummy: u8           // fill struct
}

impl WriteToBuf for TxProperties {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u32(self.flags);
        buffer.put_u8(self.dummy);
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ConfirmProperties {
    flags: u32,
    dummy: u8           // fill struct
}

impl WriteToBuf for ConfirmProperties {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u32(self.flags);
        buffer.put_u8(self.dummy);
    }
}


// frame type
pub enum FrameType {
    METHOD,
    HEADER,
    BODY,
    HEARTBEAT,
    UNKNOWN
}

impl FrameType {
    pub fn frame_type_id(&self) -> u8 {
        match self {
            FrameType::METHOD => 1,
            FrameType::HEADER => 2,
            FrameType::BODY => 3,
            FrameType::HEARTBEAT => 4,
            FrameType::UNKNOWN => 0xff
        }
    }
}

impl Default for FrameType {
    fn default() -> Self {
        FrameType::METHOD
    }
}

impl From<u8> for FrameType {
    fn from(type_id: u8) -> Self {
        match type_id {
            1 => FrameType::METHOD,
            2 => FrameType::HEADER,
            3 => FrameType::BODY,
            4 => FrameType::HEARTBEAT,
            _ => FrameType::UNKNOWN
        }
    }
}

trait MethodId {
    fn method_id(&self) -> u16;
}

pub enum ConnectionMethod {
    Start,
    StartOk,
    Secure,
    SecureOk,
    Tune,
    TuneOk,
    Open,
    OpenOk,
    Close,
    CloseOk,
    Unknown
}

impl MethodId for ConnectionMethod {
    fn method_id(&self) -> u16 {
        match self {
            ConnectionMethod::Start => 10,
            ConnectionMethod::StartOk => 11,
            ConnectionMethod::Secure => 20,
            ConnectionMethod::SecureOk => 21,
            ConnectionMethod::Tune => 30,
            ConnectionMethod::TuneOk => 31,
            ConnectionMethod::Open => 40,
            ConnectionMethod::OpenOk => 41,
            ConnectionMethod::Close => 50,
            ConnectionMethod::CloseOk => 51,
            ConnectionMethod::Unknown => 0xffff
        }
    }
}

impl Default for ConnectionMethod {
    fn default() -> Self {
        ConnectionMethod::Unknown
    }
}

impl From<u16> for ConnectionMethod {
    fn from(method_id: u16) -> Self {
        match method_id {
            10 => ConnectionMethod::Start,
            11 => ConnectionMethod::StartOk,
            20 => ConnectionMethod::Secure,
            21 => ConnectionMethod::SecureOk,
            30 => ConnectionMethod::Tune,
            31 => ConnectionMethod::TuneOk,
            40 => ConnectionMethod::Open,
            41 => ConnectionMethod::OpenOk,
            50 => ConnectionMethod::Close,
            51 => ConnectionMethod::CloseOk,
            _  => ConnectionMethod::Unknown
        }
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

pub enum AccessMethod {
    Request,
    RequestOk,
    Unknown
}

impl MethodId for AccessMethod {
    fn method_id(&self) -> u16 {
        match self {
            AccessMethod::Request => 10,
            AccessMethod::RequestOk => 11,
            AccessMethod::Unknown => 0xffff
        }
    }
}

impl Default for AccessMethod {
    fn default() -> Self {
        AccessMethod::Unknown
    }
}

impl From<u16> for AccessMethod {
    fn from(method_id: u16) -> Self {
        match method_id {
            10 => AccessMethod::Request,
            11 => AccessMethod::RequestOk,
            _  => AccessMethod::Unknown
        }
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

pub enum TxMethod {
    Select,
    SelectOk,
    Commit,
    CommitOk,
    Rollback,
    RollbackOk,
    Unknown
}

impl MethodId for TxMethod {
    fn method_id(&self) -> u16 {
        match self {
            TxMethod::Select => 10,
            TxMethod::SelectOk => 11,
            TxMethod::Commit => 20,
            TxMethod::CommitOk => 21,
            TxMethod::Rollback => 30,
            TxMethod::RollbackOk => 31,
            TxMethod::Unknown => 0xffff
        }
    }
}

impl Default for TxMethod {
    fn default() -> Self {
        TxMethod::Unknown
    }
}

impl From<u16> for TxMethod {
    fn from(method_id: u16) -> Self {
        match method_id {
            10 => TxMethod::Select,
            11 => TxMethod::SelectOk,
            20 => TxMethod::Commit,
            21 => TxMethod::CommitOk,
            30 => TxMethod::Rollback,
            31 => TxMethod::RollbackOk,
            _  => TxMethod::Unknown
        }
    }
}

pub enum Class {
    Connection,
    Channel,
    Access,
    Exchange,
    Queue,
    Basic,
    Tx,
    Unknown
}

impl Class {
    pub fn class_id(&self) -> u16 {
        match self {
            Class::Connection => 10,
            Class::Channel => 20,
            Class::Access => 30,
            Class::Exchange => 40,
            Class::Queue => 50,
            Class::Basic => 60,
            Class::Tx => 90,
            Class::Unknown => 0xffff
        }
    }
}

impl From<u16> for Class {
    fn from(class_id: u16) -> Self {
        match class_id {
            10 => Class::Connection,
            20 => Class::Channel,
            30 => Class::Access,
            40 => Class::Exchange,
            50 => Class::Queue,
            60 => Class::Basic,
            90 => Class::Tx,
            _  => Class::Unknown
        }
    }
}

impl Default for Class {
    fn default() -> Self {
        Class::Connection
    }
}

// frame end octet, every frame should end with 0xce
pub const FRAME_END: u8 = 0xce;

#[derive(Property)]
#[property(get(public), set(public))]
pub struct ProtocolHeader {
    protocol: Vec<u8>,
    major_id: u8,
    minor_id: u8,
    major_version: u8,
    minor_version: u8
}

impl WriteToBuf for ProtocolHeader {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.extend_from_slice(&self.protocol);
        buffer.put_u8(self.major_id);
        buffer.put_u8(self.minor_id);
        buffer.put_u8(self.major_version);
        buffer.put_u8(self.minor_version);
    }
}

impl Default for ProtocolHeader {
    fn default() -> Self {
        ProtocolHeader {
            protocol: Vec::from("AMQP"),
            major_id: 0,
            minor_id: 0,
            major_version: 9,
            minor_version: 1
        }
    }
}

pub enum Method {
    ConnectionMethod(ConnectionMethod),
    ChannelMethod(ChannelMethod),
    AccessMethod(AccessMethod),
    ExchangeMethod(ExchangeMethod),
    QueueMethod(QueueMethod),
    BasicMethod(BasicMethod),
    TxMethod(TxMethod)
}

impl MethodId for Method {
    fn method_id(&self) -> u16 {
        match self {
            Method::ConnectionMethod(method) => method.method_id(),
            Method::ChannelMethod(method) => method.method_id(),
            Method::AccessMethod(method) => method.method_id(),
            Method::ExchangeMethod(method) => method.method_id(),
            Method::QueueMethod(method) => method.method_id(),
            Method::BasicMethod(method) => method.method_id(),
            Method::TxMethod(method) => method.method_id()
        }
    }
}

impl Default for Method {
    fn default() -> Self {
        Method::ConnectionMethod(ConnectionMethod::default())
    }
}

pub enum Property {
    Connection(ConnectionProperties),
    Channel(ChannelProperties),
    Access(AccessProperties),
    Exchange(ExchangeProperties),
    Queue(QueueProperties),
    Basic(BasicProperties),
    Tx(TxProperties),
    Confirm(ConfirmProperties)
}

impl Default for Property {
    fn default() -> Self {
        Property::Connection(ConnectionProperties::default())
    }
}

impl WriteToBuf for Property {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        match self {
            Property::Connection(properties) => properties.write_to_buf(buffer),
            Property::Channel(properties) => properties.write_to_buf(buffer),
            Property::Access(properties) => properties.write_to_buf(buffer),
            Property::Exchange(properties) => properties.write_to_buf(buffer),
            Property::Queue(properties) => properties.write_to_buf(buffer),
            Property::Basic(properties) => properties.write_to_buf(buffer),
            Property::Tx(properties) => properties.write_to_buf(buffer),
            Property::Confirm(properties) => properties.write_to_buf(buffer)
        }
    }
}

pub enum Arguments {
    ConnectionStart(ConnectionStart),
    ConnectionStartOk(ConnectionStartOk),
    ConnectionSecure(ConnectionSecure),
    ConnectionSecureOk(ConnectionSecureOk),
    ConnectionTune(ConnectionTune),
    ConnectionTuneOk(ConnectionTuneOk),
    ConnectionOpen(ConnectionOpen),
    ConnectionOpenOk(ConnectionOpenOk),
    ConnectionClose(ConnectionClose),
    ConnectionCloseOk(ConnectionCloseOk),

    ChannelOpen(ChannelOpen),
    ChannelOpenOk(ChannelOpenOk),
    ChannelFlow(ChannelFlow),
    ChannelFlowOk(ChannelFlowOk),
    ChannelClose(ChannelClose),
    ChannelCloseOk(ChannelCloseOk),

    AccessRequest(AccessRequest),
    AccessRequestOk(AccessRequestOk),

    ExchangeDeclare(ExchangeDeclare),
    ExchangeDeclareOk(ExchangeDeclareOk),
    ExchangeDelete(ExchangeDelete),
    ExchangeDeleteOk(ExchangeDeleteOk),
    ExchangeBind(ExchangeBind),
    ExchangeBindOk(ExchangeBindOk),
    ExchangeUnbind(ExchangeUnbind),
    ExchangeUnbindOk(ExchangeUnbindOk),

    QueueDeclare(QueueDeclare),
    QueueDeclareOk(QueueDeclareOk),
    QueueBind(QueueBind),
    QueueBindOk(QueueBindOk),
    QueueUnbind(QueueUnbind),
    QueueUnbindOk(QueueUnbindOk),
    QueuePurge(QueuePurge),
    QueuePurgeOk(QueuePurgeOk),
    QueueDelete(QueueDelete),
    QueueDeleteOk(QueueDeleteOk),

    BasicQos(BasicQos),
    BasicQosOk(BasicQosOk),
    BasicConsume(BasicConsume),
    BasicConsumeOk(BasicConsumeOk),
    BasicCancel(BasicCancel),
    BasicCancelOk(BasicCancelOk),
    BasicPublish(BasicPublish),
    BasicReturn(BasicReturn),
    BasicDeliver(BasicDeliver),
    BasicGet(BasicGet),
    BasicGetOk(BasicGetOk),
    BasicGetEmpty(BasicGetEmpty),
    BasicAck(BasicAck),
    BasicReject(BasicReject),
    BasicRecoverAsync(BasicRecoverAsync),
    BasicRecover(BasicRecover),
    BasicRecoverOk(BasicRecoverOk),
    BasicNack(BasicNack),

    TxSelect(TxSelect),
    TxSelectOk(TxSelectOk),
    TxCommit(TxCommit),
    TxCommitOk(TxCommitOk),
    TxRollback(TxRollback),
    TxRollbackOk(TxRollbackOk),
    TxConfirmSelect(TxConfirmSelect),
    TxConfirmSelectOk(TxConfirmSelectOk)
}

impl WriteToBuf for Arguments {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        match self {
            Arguments::ConnectionStart(args) => args.write_to_buf(buffer),
            Arguments::ConnectionStartOk(args) => args.write_to_buf(buffer),
            Arguments::ConnectionSecure(args) => args.write_to_buf(buffer),
            Arguments::ConnectionSecureOk(args) => args.write_to_buf(buffer),
            Arguments::ConnectionTune(args) => args.write_to_buf(buffer),
            Arguments::ConnectionTuneOk(args) => args.write_to_buf(buffer),
            Arguments::ConnectionOpen(args) => args.write_to_buf(buffer),
            Arguments::ConnectionOpenOk(args) => args.write_to_buf(buffer),
            Arguments::ConnectionClose(args) => args.write_to_buf(buffer),
            Arguments::ConnectionCloseOk(args) => args.write_to_buf(buffer),

            Arguments::ChannelOpen(args) => args.write_to_buf(buffer),
            Arguments::ChannelOpenOk(args) => args.write_to_buf(buffer),
            Arguments::ChannelFlow(args) => args.write_to_buf(buffer),
            Arguments::ChannelFlowOk(args) => args.write_to_buf(buffer),
            Arguments::ChannelClose(args) => args.write_to_buf(buffer),
            Arguments::ChannelCloseOk(args) => args.write_to_buf(buffer),

            Arguments::AccessRequest(args) => args.write_to_buf(buffer),
            Arguments::AccessRequestOk(args) => args.write_to_buf(buffer),

            Arguments::ExchangeDeclare(args) => args.write_to_buf(buffer),
            Arguments::ExchangeDeclareOk(args) => args.write_to_buf(buffer),
            Arguments::ExchangeDelete(args) => args.write_to_buf(buffer),
            Arguments::ExchangeDeleteOk(args) => args.write_to_buf(buffer),
            Arguments::ExchangeBind(args) => args.write_to_buf(buffer),
            Arguments::ExchangeBindOk(args) => args.write_to_buf(buffer),
            Arguments::ExchangeUnbind(args) => args.write_to_buf(buffer),
            Arguments::ExchangeUnbindOk(args) => args.write_to_buf(buffer),

            Arguments::QueueDeclare(args) => args.write_to_buf(buffer),
            Arguments::QueueDeclareOk(args) => args.write_to_buf(buffer),
            Arguments::QueueBind(args) => args.write_to_buf(buffer),
            Arguments::QueueBindOk(args) => args.write_to_buf(buffer),
            Arguments::QueueUnbind(args) => args.write_to_buf(buffer),
            Arguments::QueueUnbindOk(args) => args.write_to_buf(buffer),
            Arguments::QueuePurge(args) => args.write_to_buf(buffer),
            Arguments::QueuePurgeOk(args) => args.write_to_buf(buffer),
            Arguments::QueueDelete(args) => args.write_to_buf(buffer),
            Arguments::QueueDeleteOk(args) => args.write_to_buf(buffer),

            Arguments::BasicQos(args) => args.write_to_buf(buffer),
            Arguments::BasicQosOk(args) => args.write_to_buf(buffer),
            Arguments::BasicConsume(args) => args.write_to_buf(buffer),
            Arguments::BasicConsumeOk(args) => args.write_to_buf(buffer),
            Arguments::BasicCancel(args) => args.write_to_buf(buffer),
            Arguments::BasicCancelOk(args) => args.write_to_buf(buffer),
            Arguments::BasicPublish(args) => args.write_to_buf(buffer),
            Arguments::BasicDeliver(args) => args.write_to_buf(buffer),
            Arguments::BasicReturn(args) => args.write_to_buf(buffer),
            Arguments::BasicGet(args) => args.write_to_buf(buffer),
            Arguments::BasicGetOk(args) => args.write_to_buf(buffer),
            Arguments::BasicGetEmpty(args) => args.write_to_buf(buffer),
            Arguments::BasicAck(args) => args.write_to_buf(buffer),
            Arguments::BasicReject(args) => args.write_to_buf(buffer),
            Arguments::BasicRecoverAsync(args) => args.write_to_buf(buffer),
            Arguments::BasicRecover(args) => args.write_to_buf(buffer),
            Arguments::BasicRecoverOk(args) => args.write_to_buf(buffer),
            Arguments::BasicNack(args) => args.write_to_buf(buffer),

            Arguments::TxSelect(args) => args.write_to_buf(buffer),
            Arguments::TxSelectOk(args) => args.write_to_buf(buffer),
            Arguments::TxCommit(args) => args.write_to_buf(buffer),
            Arguments::TxCommitOk(args) => args.write_to_buf(buffer),
            Arguments::TxRollback(args) => args.write_to_buf(buffer),
            Arguments::TxRollbackOk(args) => args.write_to_buf(buffer),
            Arguments::TxConfirmSelect(args) => args.write_to_buf(buffer),
            Arguments::TxConfirmSelectOk(args) => args.write_to_buf(buffer)
        }
    }
}

impl Default for Arguments {
    fn default() -> Self {
        Arguments::ConnectionClose(ConnectionClose::default())
    }
}

pub struct MethodPayload {
    class: Class,
    method: Method,
    args: Arguments
}

impl Default for MethodPayload {
    fn default() -> Self {
        MethodPayload {
            class: Class::default(),
            method: Method::default(),
            args: Arguments::default()
        }
    }
}

impl WriteToBuf for MethodPayload {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u16(self.class.class_id());
        buffer.put_u16(self.method.method_id());
        self.args.write_to_buf(buffer);
    }
}

pub struct ContentHeaderPayload {
    class: Class,
    weight: u16,
    body_size: u64,
    properties: Property
}

impl WriteToBuf for ContentHeaderPayload {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u16(self.class.class_id());
        buffer.put_u16(self.weight);
        buffer.put_u64(self.body_size);
        self.properties.write_to_buf(buffer);
    }
}

pub struct ContentBodyPayload {
    payload: Vec<u8>,
}

impl WriteToBuf for ContentBodyPayload {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.extend_from_slice(&self.payload);
    }
}

pub enum Payload {
    Method(MethodPayload),
    ContentHeader(ContentHeaderPayload),
    ContentBody(ContentBodyPayload)
}

impl Default for Payload {
    fn default() -> Self {
        Payload::Method(MethodPayload::default())
    }
}

impl WriteToBuf for Payload {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        match self {
            Payload::Method(method) => method.write_to_buf(buffer),
            Payload::ContentHeader(content_header) => content_header.write_to_buf(buffer),
            Payload::ContentBody(content_body) => content_body.write_to_buf(buffer)
        }
    }
}

// frame
#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct Frame {
    frame_type: FrameType,
    channel: u16,
    length: u32,
    payload: Payload,
}

impl WriteToBuf for Frame {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u8(self.frame_type.frame_type_id());
        buffer.put_u16(self.channel);
        buffer.put_u32(self.length);
        self.payload.write_to_buf(buffer);
        buffer.put_u8(FRAME_END);
    }
}
