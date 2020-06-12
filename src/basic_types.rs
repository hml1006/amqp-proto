use bytes::{BytesMut, BufMut};
use crate::error;

pub type Octet = u8;
pub type Short = u16;
pub type Long = u32;
pub type LongLong = u64;
pub type Timestamp = u64;

pub trait AsBytes {
    fn as_bytes(&self) -> &[u8];
}

pub struct ShortStr (BytesMut);
const SHORT_STR_LEN_SIZE: usize = std::mem::size_of::<Octet>();
impl ShortStr {

    // build a ShortStr from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<ShortStr, crate::error::Error> {
        if bytes.len() > 0xff as usize {
            Err(crate::error::Error::from(error::ErrorKind::StrTooLong))
        } else {
            let mut content = BytesMut::with_capacity(bytes.len() + SHORT_STR_LEN_SIZE);
            content.put_u8(bytes.len() as u8);
            content.extend_from_slice(bytes);
            Ok(ShortStr(content))
        }
    }
}

impl AsBytes for ShortStr {
    // convert ShortStr to bytes array
    #[inline]
    fn as_bytes(&self) -> &[u8] {
        &self.0[..]
    }
}

pub struct LongStr (BytesMut);
const LONG_STR_LEN_SIZE: usize = std::mem::size_of::<Long>();
impl LongStr {
    // build a LongStr from bytes, the length will be convert to big endian
    pub fn from_bytes(bytes: &[u8]) -> Result<LongStr, crate::error::Error> {
        if bytes.len() > 0xffffffff as usize {
            Err(crate::error::Error::from(error::ErrorKind::StrTooLong))
        } else {
            let mut content = BytesMut::with_capacity(bytes.len() + LONG_STR_LEN_SIZE);
            // u32 will put with big endian
            content.put_u32(bytes.len() as u32);
            content.extend_from_slice(bytes);
            Ok(LongStr(content))
        }
    }
}

impl AsBytes for LongStr {
    fn as_bytes(&self) -> &[u8] {
        &self.0[..]
    }
}