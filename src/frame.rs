
// frame type
pub struct FrameType {}

impl FrameType {
    pub const METHOD:u8 = 1u8;
    pub const HEADER:u8 = 2u8;
    pub const BODY:u8 = 3u8;
    pub const HEARTBEAT:u8 = 4u8;
}

// frame end octet, every frame should end with 0xce
pub const FRAME_END:u8 = 0xce;

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