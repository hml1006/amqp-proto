use property::Property;
use bytes::{BytesMut, BufMut};
use crate::common::{WriteToBuf, MethodId, Class, Method};
use crate::connection::{ConnectionClose, ConnectionCloseOk, ConnectionOpenOk, ConnectionOpen, ConnectionTuneOk, ConnectionTune, ConnectionSecureOk, ConnectionSecure, ConnectionStartOk, ConnectionStart, ConnectionProperties};
use crate::tx::{TxRollbackOk, TxRollback, TxCommitOk, TxCommit, TxSelectOk, TxSelect, TxProperties};
use crate::basic::{BasicNack, BasicRecoverOk, BasicRecover, BasicRecoverAsync, BasicReject, BasicAck, BasicGetEmpty, BasicGetOk, BasicGet, BasicDeliver, BasicReturn, BasicPublish, BasicCancelOk, BasicCancel, BasicConsumeOk, BasicConsume, BasicQosOk, BasicQos, BasicProperties};
use crate::queue::{QueueDeleteOk, QueueDelete, QueuePurgeOk, QueuePurge, QueueUnbindOk, QueueUnbind, QueueBindOk, QueueBind, QueueDeclareOk, QueueDeclare, QueueProperties};
use crate::exchange::{ExchangeUnbindOk, ExchangeUnbind, ExchangeBindOk, ExchangeBind, ExchangeDeleteOk, ExchangeDelete, ExchangeDeclareOk, ExchangeDeclare, ExchangeProperties};
use crate::access::{AccessRequestOk, AccessRequest, AccessProperties};
use crate::channel::{ChannelCloseOk, ChannelClose, ChannelFlowOk, ChannelFlow, ChannelOpenOk, ChannelOpen, ChannelProperties};
use crate::confirm::{ConfirmSelect, ConfirmSelectOk, ConfirmProperties};

// frame end octet, every frame should end with 0xce
pub const FRAME_END: u8 = 0xce;

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

    ConfirmSelect(ConfirmSelect),
    ConfirmSelectOk(ConfirmSelectOk)
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

            Arguments::ConfirmSelect(args) => args.write_to_buf(buffer),
            Arguments::ConfirmSelectOk(args) => args.write_to_buf(buffer)
        }
    }
}

impl Default for Arguments {
    fn default() -> Self {
        Arguments::ConnectionClose(ConnectionClose::default())
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct MethodPayload {
    class: Class,
    method: Method,
    args: Arguments
}

impl WriteToBuf for MethodPayload {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u16(self.class.class_id());
        buffer.put_u16(self.method.method_id());
        self.args.write_to_buf(buffer);
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
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

#[derive(Property, Default)]
#[property(get(public), set(public))]
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
