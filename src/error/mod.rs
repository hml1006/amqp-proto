
pub mod amqp;

mod frame;
pub use frame::FrameDecodeErr;
use nom::error::ErrorKind;

pub type NomErr<'a> = (&'a [u8], ErrorKind);