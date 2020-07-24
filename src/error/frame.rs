use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum FrameDecodeErr {
    Incomplete,
    SyntaxError(&'static str),
    DecodeError(String),
}

impl Display for FrameDecodeErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            FrameDecodeErr::Incomplete => write!(f, "Incomplete"),
            FrameDecodeErr::SyntaxError(e) => write!(f, "Syntax error: {}", e),
            FrameDecodeErr::DecodeError(e) => write!(f, "Decode error while -> {}", e)
        }
    }
}

impl std::error::Error for FrameDecodeErr {}

impl From<io::Error> for FrameDecodeErr {
    fn from(e: io::Error) -> Self {
        FrameDecodeErr::DecodeError(format!("found io error: {}", e))
    }
}