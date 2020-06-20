use crate::{FieldTable, LongStr};

// frame type
pub enum FrameType {
    METHOD = 1,
    HEADER = 2,
    BODY = 3,
    HEARTBEAT = 4
}

// frame end octet, every frame should end with 0xce
pub const FRAME_END: u8 = 0xce;

pub struct ProtocolHeader {
    protocol: [u8; 4],
    major_id: u8,
    minor_id: u8,
    major_version: u8,
    minor_version: u8
}

// default protocol header
pub const PROTOCOL_HEADER: ProtocolHeader = ProtocolHeader {
    protocol: [b'A', b'M', b'Q', b'P'],
    major_id: 0,
    minor_id: 0,
    major_version: 9,
    minor_version: 1
};

pub struct MethodPayload {}
pub struct ContentHeaderPayload {}
pub struct ContentBodyPayload {}

pub enum Payload {
    Method(MethodPayload),
    Header(ContentHeaderPayload),
    Body(ContentBodyPayload)
}

// frame
pub struct Frame {
    frame_type: u8,
    channel: u16,
    length: u32,
    payload: Payload,
    frame_end: u8
}

pub struct ConnectionStart {
    version_major: u8,
    version_minor: u8,
    server_properties: FieldTable,
    mechanisms: LongStr,
    locales: LongStr,
}

pub struct ConnectionStartOk {
    client_properties: FieldTable,
    mechanisms: LongStr,
    response: LongStr,
    locales: LongStr
}

pub struct ConnectionSecure {
    challenge: LongStr
}

pub struct ConnectionSecureOk {
    response: LongStr
}

pub struct ConnectionTunre {
    channel_max: u16,
    frame_max: u32,
    heartbeat: u16
}

pub struct ConnectionTuneOk {
    channel_max: u16,
    frame_max: u32,
    heartbeat: u16
}

pub struct ConnectionOpen {
    vhost: LongStr,
    capabilities: LongStr,  // rabbitmq used
    insist: bool
}

pub struct ConnectionOpenOk {
    known_hosts: LongStr
}

pub struct ConnectionClose {
    reply_code: u16,
    reply_text: LongStr,
    class_id: u16,
    method_id: u16
}

pub struct ConnectionCloseOk {
    dummy: u8               // fill struct
}

pub struct ChannelOpen {
    out_of_band: LongStr
}

pub struct ChannelOpenOk {
    channel_id: LongStr
}

pub struct ChannelFlow {
    active: bool
}

pub struct ChannelFlowOk {
    active: bool
}

pub struct ChannelClose {
    reply_code: u16,
    reply_text: LongStr,
    class_id: u16,
    method_id: u16
}

pub struct ChannelCloseOk {
    dummy: u8           // fill struct
}