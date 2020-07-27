
pub mod amqp;

mod frame;
pub use frame::FrameDecodeErr;
use nom::error::ErrorKind;

pub(crate) type NomErr<'a> = (&'a [u8], ErrorKind);