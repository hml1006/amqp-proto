use tokio_util::codec::Decoder;
use bytes::BytesMut;
use crate::error::FrameDecodeErr;
use crate::frame::base::{ProtocolHeader, Frame};
use crate::frame::frame_codec::DecodedFrame::AmqpFrame;
use crate::codec::Decode;

pub const PROTOCOL_HEADER_SIZE: usize = 8;

pub enum DecodedFrame {
    ProtocolHeader(ProtocolHeader),
    AmqpFrame(Frame)
}

pub struct FrameCodec {
    header_received: bool,
}

impl Default for FrameCodec {
    fn default() -> Self {
        FrameCodec {
            header_received: false,
        }
    }
}

impl Decoder for FrameCodec {
    type Item = DecodedFrame;
    type Error = FrameDecodeErr;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        // parse amqp header
        if !self.header_received {
            match ProtocolHeader::decode(src) {
                Ok((_, header)) => {
                    let _ = src.split_to(PROTOCOL_HEADER_SIZE);
                    return Ok(Some(DecodedFrame::ProtocolHeader(header)))
                },
                Err(e) => {
                    match e {
                        FrameDecodeErr::Incomplete => return Ok(None),
                        _ => return Err(FrameDecodeErr::DecodeError(format!("codec decode ProtocolHeader failed -> {}", e)))
                    }
                }
            }
        }

        // +-frame type: u8-+---channel id: u16---+-----length: u32-----+----payload---+--frame end--+
        // |   1|2|3|4      |       0x0000        |     payload length  |              |  0xce       |
        // +----------------+---------------------+---------------------+--------------+-------------+
        match Frame::decode(&src[..]) {
            Ok((_, frame)) => {
                let _ = src.split_to(frame.len());
                Ok(Some(AmqpFrame(frame)))
            }
            Err(e) => {
                match e {
                    FrameDecodeErr::Incomplete => Ok(None),
                    _ => return Err(FrameDecodeErr::DecodeError(format!("codec decode Frame failed -> {}", e)))
                }
            }
        }
    }
}
