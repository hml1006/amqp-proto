use property::Property;
use bytes::{BytesMut, BufMut};
use crate::common::{Encode, MethodId, Class, Method};
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

impl Encode for ProtocolHeader {
    fn encode(&self, buffer: &mut BytesMut) {
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

impl Encode for Property {
    fn encode(&self, buffer: &mut BytesMut) {
        match self {
            Property::Connection(properties) => properties.encode(buffer),
            Property::Channel(properties) => properties.encode(buffer),
            Property::Access(properties) => properties.encode(buffer),
            Property::Exchange(properties) => properties.encode(buffer),
            Property::Queue(properties) => properties.encode(buffer),
            Property::Basic(properties) => properties.encode(buffer),
            Property::Tx(properties) => properties.encode(buffer),
            Property::Confirm(properties) => properties.encode(buffer)
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

impl Encode for Arguments {
    fn encode(&self, buffer: &mut BytesMut) {
        match self {
            Arguments::ConnectionStart(args) => args.encode(buffer),
            Arguments::ConnectionStartOk(args) => args.encode(buffer),
            Arguments::ConnectionSecure(args) => args.encode(buffer),
            Arguments::ConnectionSecureOk(args) => args.encode(buffer),
            Arguments::ConnectionTune(args) => args.encode(buffer),
            Arguments::ConnectionTuneOk(args) => args.encode(buffer),
            Arguments::ConnectionOpen(args) => args.encode(buffer),
            Arguments::ConnectionOpenOk(args) => args.encode(buffer),
            Arguments::ConnectionClose(args) => args.encode(buffer),
            Arguments::ConnectionCloseOk(args) => args.encode(buffer),

            Arguments::ChannelOpen(args) => args.encode(buffer),
            Arguments::ChannelOpenOk(args) => args.encode(buffer),
            Arguments::ChannelFlow(args) => args.encode(buffer),
            Arguments::ChannelFlowOk(args) => args.encode(buffer),
            Arguments::ChannelClose(args) => args.encode(buffer),
            Arguments::ChannelCloseOk(args) => args.encode(buffer),

            Arguments::AccessRequest(args) => args.encode(buffer),
            Arguments::AccessRequestOk(args) => args.encode(buffer),

            Arguments::ExchangeDeclare(args) => args.encode(buffer),
            Arguments::ExchangeDeclareOk(args) => args.encode(buffer),
            Arguments::ExchangeDelete(args) => args.encode(buffer),
            Arguments::ExchangeDeleteOk(args) => args.encode(buffer),
            Arguments::ExchangeBind(args) => args.encode(buffer),
            Arguments::ExchangeBindOk(args) => args.encode(buffer),
            Arguments::ExchangeUnbind(args) => args.encode(buffer),
            Arguments::ExchangeUnbindOk(args) => args.encode(buffer),

            Arguments::QueueDeclare(args) => args.encode(buffer),
            Arguments::QueueDeclareOk(args) => args.encode(buffer),
            Arguments::QueueBind(args) => args.encode(buffer),
            Arguments::QueueBindOk(args) => args.encode(buffer),
            Arguments::QueueUnbind(args) => args.encode(buffer),
            Arguments::QueueUnbindOk(args) => args.encode(buffer),
            Arguments::QueuePurge(args) => args.encode(buffer),
            Arguments::QueuePurgeOk(args) => args.encode(buffer),
            Arguments::QueueDelete(args) => args.encode(buffer),
            Arguments::QueueDeleteOk(args) => args.encode(buffer),

            Arguments::BasicQos(args) => args.encode(buffer),
            Arguments::BasicQosOk(args) => args.encode(buffer),
            Arguments::BasicConsume(args) => args.encode(buffer),
            Arguments::BasicConsumeOk(args) => args.encode(buffer),
            Arguments::BasicCancel(args) => args.encode(buffer),
            Arguments::BasicCancelOk(args) => args.encode(buffer),
            Arguments::BasicPublish(args) => args.encode(buffer),
            Arguments::BasicDeliver(args) => args.encode(buffer),
            Arguments::BasicReturn(args) => args.encode(buffer),
            Arguments::BasicGet(args) => args.encode(buffer),
            Arguments::BasicGetOk(args) => args.encode(buffer),
            Arguments::BasicGetEmpty(args) => args.encode(buffer),
            Arguments::BasicAck(args) => args.encode(buffer),
            Arguments::BasicReject(args) => args.encode(buffer),
            Arguments::BasicRecoverAsync(args) => args.encode(buffer),
            Arguments::BasicRecover(args) => args.encode(buffer),
            Arguments::BasicRecoverOk(args) => args.encode(buffer),
            Arguments::BasicNack(args) => args.encode(buffer),

            Arguments::TxSelect(args) => args.encode(buffer),
            Arguments::TxSelectOk(args) => args.encode(buffer),
            Arguments::TxCommit(args) => args.encode(buffer),
            Arguments::TxCommitOk(args) => args.encode(buffer),
            Arguments::TxRollback(args) => args.encode(buffer),
            Arguments::TxRollbackOk(args) => args.encode(buffer),

            Arguments::ConfirmSelect(args) => args.encode(buffer),
            Arguments::ConfirmSelectOk(args) => args.encode(buffer)
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

impl Encode for MethodPayload {
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.put_u16(self.class.class_id());
        buffer.put_u16(self.method.method_id());
        self.args.encode(buffer);
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

impl Encode for ContentHeaderPayload {
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.put_u16(self.class.class_id());
        buffer.put_u16(self.weight);
        buffer.put_u64(self.body_size);
        self.properties.encode(buffer);
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct ContentBodyPayload {
    payload: Vec<u8>,
}

impl Encode for ContentBodyPayload {
    fn encode(&self, buffer: &mut BytesMut) {
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

impl Encode for Payload {
    fn encode(&self, buffer: &mut BytesMut) {
        match self {
            Payload::Method(method) => method.encode(buffer),
            Payload::ContentHeader(content_header) => content_header.encode(buffer),
            Payload::ContentBody(content_body) => content_body.encode(buffer)
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

impl Encode for Frame {
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.put_u8(self.frame_type.frame_type_id());
        buffer.put_u16(self.channel);
        buffer.put_u32(self.length);
        self.payload.encode(buffer);
        buffer.put_u8(FRAME_END);
    }
}
