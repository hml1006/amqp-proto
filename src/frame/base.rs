use property::Property;
use bytes::{BytesMut, BufMut};
use std::collections::HashMap;
use std::hash::{Hasher, Hash};
use std::vec::Vec;
use std::string::String;
use nom::number::streaming::{be_i8, be_u8, be_i16, be_u16, be_i32, be_u32, be_u64, be_i64, be_f32, be_f64};
use nom::bytes::streaming::take;
use nom::error::ErrorKind;
use crate::error::{NomErr, FrameDecodeErr};
use crate::frame::header::connection::ConnectionProperties;
use crate::frame::header::channel::ChannelProperties;
use crate::frame::header::access::AccessProperties;
use crate::frame::header::exchange::ExchangeProperties;
use crate::frame::header::queue::QueueProperties;
use crate::frame::header::basic::BasicProperties;
use crate::frame::header::tx::TxProperties;
use crate::frame::method::connection::{ConnectionStart, ConnectionStartOk, ConnectionSecure, ConnectionSecureOk, ConnectionTune, ConnectionTuneOk, ConnectionOpen, ConnectionOpenOk, ConnectionClose, ConnectionCloseOk};
use crate::frame::method::channel::{ChannelOpen, ChannelOpenOk, ChannelFlow, ChannelFlowOk, ChannelClose, ChannelCloseOk};
use crate::frame::method::access::{AccessRequest, AccessRequestOk};
use crate::frame::method::exchange::{ExchangeDeclare, ExchangeDelete, ExchangeDeclareOk, ExchangeDeleteOk, ExchangeBind, ExchangeBindOk, ExchangeUnbind, ExchangeUnbindOk};
use crate::frame::method::queue::{QueueDeclare, QueueBind, QueueBindOk, QueueDeclareOk, QueueUnbind, QueueUnbindOk, QueuePurge, QueuePurgeOk, QueueDelete, QueueDeleteOk};
use crate::frame::method::basic::{BasicQos, BasicQosOk, BasicConsume, BasicConsumeOk, BasicCancel, BasicCancelOk, BasicPublish, BasicReturn, BasicDeliver, BasicGet, BasicGetOk, BasicGetEmpty, BasicAck, BasicReject, BasicRecoverAsync, BasicRecover, BasicRecoverOk, BasicNack};
use crate::frame::method::tx::{TxSelect, TxSelectOk, TxCommit, TxCommitOk, TxRollback, TxRollbackOk};
use crate::frame::method::confirm::{ConfirmSelect, ConfirmSelectOk};
use crate::class::Class;
use crate::method::{Method, MethodId, get_method_type, ConnectionMethod, ChannelMethod, AccessMethod, ExchangeMethod, QueueMethod, BasicMethod, TxMethod, ConfirmMethod};
use crate::frame::header::confirm::ConfirmProperties;

// amqp0-9-1 field name length allowed is 128
const MAX_FIELD_NAME_LEN: usize = 128;
// max long string bytes length allowed
const MAX_LONG_STR_LEN: usize = 64 * 1024;

pub trait Encode {
    // write data to bytes buffer
    fn encode(&self, buffer: &mut BytesMut);
}

pub trait Decode<T> {
    // parse data from bytes buffer
    fn decode(buffer: &[u8]) -> Result<(&[u8], T), FrameDecodeErr>;
}

// impl Encode for primitive types
macro_rules! encode_impl_for_primitive {
    ($($t:ty)*) => {$(
        paste::item! {
            impl Encode for $t {
                #[inline]
                fn encode(&self, buffer: &mut BytesMut) {
                    buffer.[<put_ $t>](*self);
                }
            }
        }
    )*}
}
encode_impl_for_primitive!(u8 i8 u16 i16 u32 i32 u64 i64 f32 f64);

// impl for primitive types
macro_rules! decode_impl_for_primitive {
    ($($t:ty)*) => {$(
        paste::item! {
            impl Decode<$t> for $t {
                #[inline]
                fn decode(buffer: &[u8]) -> Result<(&[u8], $t), FrameDecodeErr> {
                    match [<be_ $t>]::<(_, ErrorKind)>(buffer) {
                        Ok(v) => Ok(v),
                        Err(e) => {
                            match e {
                                nom::Err::Incomplete(_) => return Err(FrameDecodeErr::Incomplete),
                                _ => return Err(FrameDecodeErr::DecodeError(format!("decode primitive -> {}", e)))
                            }
                        }
                    }
                }
            }
        }
    )*}
}
decode_impl_for_primitive!(u8 i8 u16 i16 u32 i32 u64 i64 f32 f64);

pub(crate) fn take_bytes(buffer: &[u8], count: usize) -> Result<(&[u8], &[u8]), FrameDecodeErr> {
    match take::<usize, &[u8], NomErr>(count)(buffer) {
        Ok(v) => Ok(v),
        Err(e) => {
            match e {
                nom::Err::Incomplete(_) => return Err(FrameDecodeErr::Incomplete),
                _ => return Err(FrameDecodeErr::DecodeError(format!("take bytes -> {}", e)))
            }
        }
    }
}

pub type Timestamp = u64;

#[derive(Default, Debug, PartialEq, Eq)]
pub struct ShortStr (String);

impl std::hash::Hash for ShortStr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl ToString for ShortStr {
    #[inline]
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl ShortStr {
    /// Create a ShortStr from bytes
    ///
    /// # Examples
    /// ```
    /// use amqp_proto::ShortStr;
    ///
    /// let bytes = b"hello";
    /// let short_str = ShortStr::with_bytes(bytes).unwrap();
    ///
    /// assert_eq!(short_str.to_string(), String::from("hello"));
    /// ```
    #[inline]
    pub fn with_bytes(bytes: &[u8]) -> Result<Self, FrameDecodeErr>{
        if bytes.len() > std::u8::MAX as usize {
            return Err(FrameDecodeErr::SyntaxError("ShortStr too long"));
        }
        Ok(ShortStr(String::from_utf8_lossy(bytes).to_string()))
    }
}

impl Encode for ShortStr {
    /// Write bytes to BytesMut
    ///
    /// # Examples
    /// ```
    /// use amqp_proto::ShortStr;
    /// use bytes::BytesMut;
    /// use amqp_proto::codec::Encode;
    ///
    /// let short_str = ShortStr::with_bytes(b"hello").unwrap();
    /// let mut buffer = BytesMut::with_capacity(64);
    ///
    /// short_str.encode(&mut buffer);
    ///
    /// assert_eq!(&buffer[..], &[5u8, 104, 101, 108, 108, 111]);
    ///
    /// ```
    #[inline]
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.put_u8(self.0.len() as u8);
        buffer.extend_from_slice(&self.0.as_bytes());
    }
}

impl Decode<ShortStr> for ShortStr {
    /// Decode ShortStr from bytes.
    ///
    /// # Examples
    /// ```
    /// use amqp_proto::ShortStr;
    /// use amqp_proto::codec::Decode;
    ///
    /// let (_, short_str) = ShortStr::decode(&[5u8, 104, 101, 108, 108, 111]).unwrap();
    ///
    /// assert_eq!(short_str.to_string(), String::from("hello"));
    /// ```
    fn decode(buffer: &[u8]) -> Result<(&[u8], ShortStr), FrameDecodeErr> {
        let (buffer, length) = match u8::decode(buffer) {
            Ok(v) => v,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode ShortStr length -> {}", e)))
        };
        let (buffer, data) = match take_bytes(buffer, length as usize) {
            Ok(v) => v,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode ShortStr bytes -> {}", e)))
        };
        let short_str = match ShortStr::with_bytes(data) {
            Ok(short_str) => short_str,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode but build ShortStr failed -> {}", e)))
        };
        Ok((buffer, short_str))
    }
}

#[derive(Debug, Default, Clone)]
pub struct LongStr(String);

impl ToString for LongStr {
    #[inline]
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl LongStr {
    /// Create a LongStr from bytes, the length will be convert to big endian
    ///
    /// # Examples
    ///
    /// ```
    /// use amqp_proto::LongStr;
    ///
    /// let long_string = LongStr::with_bytes(b"hello").unwrap();
    ///
    ///  assert_eq!(long_string.to_string(), String::from("hello"));
    /// ```
    #[inline]
    pub fn with_bytes(bytes: &[u8]) -> Result<LongStr, FrameDecodeErr> {
        if bytes.len() > MAX_LONG_STR_LEN {
            Err(FrameDecodeErr::SyntaxError("LongStr too long"))
        } else {
            Ok(LongStr(String::from_utf8_lossy(bytes).to_string()))
        }
    }
}

impl Encode for LongStr {
    /// Write bytes to BytesMut
    ///
    /// # Examples
    ///
    /// ```
    /// use amqp_proto::LongStr;
    /// use bytes::BytesMut;
    /// use amqp_proto::codec::Encode;
    ///
    /// let long_string = LongStr::with_bytes(b"hello").unwrap();
    /// let mut buffer = BytesMut::with_capacity(64);
    ///
    /// long_string.encode(&mut buffer);
    ///
    /// assert_eq!(&buffer[..], &[0, 0, 0, 5u8, 104, 101, 108, 108, 111])
    /// ```
    #[inline]
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.put_u32(self.0.len() as u32);
        buffer.extend_from_slice(self.0.as_bytes());
    }
}

impl Decode<LongStr> for LongStr {
    /// Decode LongStr from bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use amqp_proto::LongStr;
    /// use amqp_proto::codec::Decode;
    ///
    /// let (_, long_string) = LongStr::decode(&[0, 0, 0, 5u8, 104, 101, 108, 108, 111]).unwrap();
    ///
    /// assert_eq!(long_string.to_string(), String::from("hello"));
    /// ```
    fn decode(buffer: &[u8]) -> Result<(&[u8], LongStr), FrameDecodeErr> {
        let (buffer, length) = match u32::decode(buffer) {
            Ok(v) => v,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode LongStr length -> {}", e)))
        };
        let (buffer, data) = match take_bytes(buffer, length as usize) {
            Ok(v) => v,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode LongStr bytes -> {}", e)))
        };
        let long_str = match LongStr::with_bytes(data) {
            Ok(long_str) => long_str,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode but build LongStr failed -> {}", e)))
        };
        Ok((buffer, long_str))
    }
}

pub type ByteArray = LongStr;

#[derive(Debug, PartialEq, Eq)]
pub struct Decimal {
    scale: u8,
    value: u32
}

impl Decimal {
    #[inline]
    pub fn new( scale: u8, value: u32) -> Self {
        Decimal { scale, value }
    }
}

impl Encode for Decimal {
    /// Encode Decimal to BytesMut
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytes::BytesMut;
    /// use amqp_proto::Decimal;
    /// use amqp_proto::codec::Encode;
    ///
    /// let decimal = Decimal::new(1,5);
    /// let mut buffer = BytesMut::with_capacity(8);
    ///
    /// decimal.encode(&mut buffer);
    ///
    /// assert_eq!(&buffer[..], &[1u8, 0, 0, 0, 5]);
    /// ```
    #[inline]
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.put_u8(self.scale);
        buffer.put_u32(self.value);
    }
}

impl Decode<Decimal> for Decimal {
    /// Decode decimal from bytes
    ///
    /// # Examples
    ///
    /// ```rust
    /// use amqp_proto::Decimal;
    /// use amqp_proto::codec::Decode;
    ///
    /// let (_, decimal) = Decimal::decode(&[1u8, 0, 0, 0, 5]).unwrap();
    ///
    /// assert_eq!(decimal, Decimal::new(1, 5));
    /// ```
    #[inline]
    fn decode(buffer: &[u8]) -> Result<(&[u8], Decimal), FrameDecodeErr> {
        let (buffer, scale) = match u8::decode(buffer) {
            Ok(v) => v,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode Decimal scale -> {}", e)))
        };
        let (buffer, value) = match u32::decode(buffer) {
            Ok(v) => v,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode Decimal value -> {}", e)))
        };
        Ok((buffer, Decimal { scale, value }))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct FieldName(ShortStr);

impl ToString for FieldName {
    #[inline]
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl FieldName {
    /// Create FieldName from bytes.
    ///
    /// # Examples
    /// ```rust
    /// use amqp_proto::FieldName;
    ///
    /// let field_name = FieldName::with_bytes(b"2ello").unwrap_err();
    /// assert!(format!("{}", field_name).contains("FieldName start char error"));
    ///
    /// let field_name = FieldName::with_bytes(b"$ello").unwrap();
    /// assert_eq!(field_name.to_string(), String::from("$ello"));
    /// ```
    #[inline]
    pub fn with_bytes(bytes: &[u8]) -> Result<FieldName, FrameDecodeErr> {
        // field name first letter should be '$'  '#' or letter
        let is_start_char_ok = match bytes[0] {
            b'$' | b'#' => true,
            b'a'..=b'z' | b'A'..=b'Z' => true,
            _ => false
        };

        if !is_start_char_ok {
            return Err(FrameDecodeErr::SyntaxError("FieldName start char error"));
        }

        // max field name length is 128
        if bytes.len() > MAX_FIELD_NAME_LEN {
            return Err(FrameDecodeErr::SyntaxError("FieldName field name length too long"));
        }

        match ShortStr::with_bytes(bytes) {
            Ok(value) => Ok(FieldName(value)),
            Err(e) => Err(FrameDecodeErr::DecodeError(format!("build FieldName failed -> {}", e)))
        }
    }
}

impl Hash for FieldName {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl Encode for FieldName {
    #[inline]
    fn encode(&self, buffer: &mut BytesMut) {
        self.0.encode(buffer);
    }
}

impl Decode<FieldName> for FieldName {
    /// Decode FieldName from bytes
    ///
    /// # Examples
    ///
    /// ```rust
    /// use amqp_proto::FieldName;
    /// use amqp_proto::codec::Decode;
    ///
    /// let (_, field_name) = FieldName::decode(&[5u8, 104, 101, 108, 108, 111]).unwrap();
    /// assert_eq!(field_name.to_string(), String::from("hello"));
    ///
    /// let err = FieldName::decode(&[5u8, 104, 101, 108, 108]).unwrap_err();
    /// assert!(format!("{}", err).contains("decode FieldName bytes"));
    /// ```
    #[inline]
    fn decode(buffer: &[u8]) -> Result<(&[u8], FieldName), FrameDecodeErr>{
        let (buffer, length) = match u8::decode(buffer) {
            Ok(v) => v,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode FieldName length -> {}", e)))
        };
        let (buffer, data) = match take_bytes(buffer, length as usize) {
            Ok(v) => v,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode FieldName bytes -> {}", e)))
        };

        match FieldName::with_bytes(data) {
            Ok(v) => Ok((buffer, v)),
            Err(e) => Err(FrameDecodeErr::DecodeError(format!("decode but build FieldName failed -> {}", e)))
        }
    }
}

pub type FieldArray = Vec<FieldValue>;

impl Encode for FieldArray {
    /// Encode FieldArray to bytes.
    ///
    /// # Examples
    /// ```rust
    /// use amqp_proto::{FieldArray, FieldValue};
    /// use amqp_proto::codec::Encode;
    /// use bytes::BytesMut;
    ///
    /// let mut buffer = BytesMut::with_capacity(64);
    /// let mut arr = FieldArray::new();
    /// arr.push(FieldValue::from_u8(0x1));
    /// arr.push(FieldValue::from_u8(0x2));
    /// arr.push(FieldValue::from_u8(0x3));
    /// buffer.clear();
    /// arr.encode(&mut buffer);
    /// assert_eq!(&buffer[..], &[0x0u8, 0, 0, 0x6u8, b'B', 0x1, b'B', 0x2, b'B', 0x3]);
    /// ```
    fn encode(&self, buffer: &mut BytesMut) {
        let mut index = buffer.len();
        buffer.put_u32(0);
        for item in self {
            item.encode(buffer);
        }
        let field_table_len = (buffer.len() - index - std::mem::size_of::<u32>()) as u32;
        // set the true length of the field table
        for i in &field_table_len.to_be_bytes() {
            buffer[index] = *i;
            index += 1;
        }
    }
}

impl Decode<FieldArray> for FieldArray {
    /// Encode FieldArray to bytes.
    ///
    /// # Examples
    /// ```rust
    /// use amqp_proto::{FieldArray, FieldValue};
    /// use amqp_proto::codec::{Encode, Decode};
    /// use bytes::BytesMut;
    ///
    /// let (_, arr) = FieldArray::decode(&[0x0u8, 0, 0, 14u8, b'B', 0x1, b'B', 0x2, b'S', 0, 0, 0, 5u8, 104, 101, 108, 108, 111]).unwrap();
    /// assert!(matches!(arr[0], FieldValue::U8(v) if v == 0x1u8));
    /// assert!(matches!(arr[1], FieldValue::U8(v) if v == 0x2u8));
    /// assert!(matches!(arr[2], FieldValue::LongStr(ref v) if v.to_string() == String::from("hello")));
    /// ```
    fn decode(buffer: &[u8]) -> Result<(&[u8], FieldArray), FrameDecodeErr> {
        // array bytes length
        let (buffer, length) = match u32::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode FieldArray length -> {}", e)))
        };

        // array bytes
        let (buffer, data) = match take_bytes(buffer, length as usize) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode FieldArray bytes->{}", e)))
        };

        let mut arr: Vec<FieldValue> = Vec::new();
        let mut tmp = data;
        loop {
            let (retain, value) = match FieldValue::decode(tmp) {
                Ok(ret) => ret,
                Err(e) => return Err(FrameDecodeErr::DecodeError(format!("read FieldArray item failed -> {}", e)))
            };
            tmp = retain;
            arr.push(value);
            if tmp.len() == 0 {
                return Ok((buffer, arr))
            }
        }
    }
}

pub type BytesArray = LongStr;

#[derive(Debug)]
pub enum FieldValueKind {
    Boolean,        // 0 = False, else True
    I8,             // Octet
    U8,             // Octet
    I16,            // 2 * Octet, same as rabbitmq
    U16,            // u16
    I32,
    U32,
    I64,            // same as rabbitmq
    U64,            // rabbitmq hasn't this field type
    F32,            // 4 * Octet
    F64,            // 8 * Octet
    Timestamp,      // u64
    Decimal,        // scale long-uint, u8 + u32, scale is pointer position
    // ShortStr,    // rabbitmq hasn't this field type
    LongStr,        // UTF-8 null-terminated character string, u32 + content
    FieldArray,     // length + field value + filed value +...
    FieldTable,     // nested field table
    BytesArray,      // same as rabbitmq, len + bytes
    Void,           // no field
    Unknown
}

impl FieldValueKind {
    #[inline]
    pub fn as_u8(&self) -> u8 {
        match self {
            FieldValueKind::Boolean=> b't',
            FieldValueKind::I8 => b'b',
            FieldValueKind::U8 => b'B',
            FieldValueKind::I16 => b's',
            FieldValueKind::U16 => b'u',
            FieldValueKind::I32 => b'I',
            FieldValueKind::U32 => b'i',
            FieldValueKind::I64 => b'l',
            FieldValueKind::U64 => b'L',
            FieldValueKind::F32 => b'f',
            FieldValueKind::F64 => b'd',
            FieldValueKind::Timestamp => b'T',
            FieldValueKind::Decimal => b'D',
            FieldValueKind::LongStr => b'S',
            FieldValueKind::FieldArray => b'A',
            FieldValueKind::FieldTable => b'F',
            FieldValueKind::BytesArray => b'x',
            FieldValueKind::Void => b'V',
            FieldValueKind::Unknown => 0xff
        }
    }
}

impl From<u8> for FieldValueKind {
    fn from(tag: u8) -> Self {
        match tag {
            b't' => FieldValueKind::Boolean,
            b'b' => FieldValueKind::I8,
            b'B' => FieldValueKind::U8,
            b's' => FieldValueKind::I16,
            b'u' => FieldValueKind::U16,
            b'I' => FieldValueKind::I32,
            b'i' => FieldValueKind::U32,
            b'l' => FieldValueKind::I64,
            b'L' => FieldValueKind::U64,
            b'f' => FieldValueKind::F32,
            b'd' => FieldValueKind::F64,
            b'T' => FieldValueKind::Timestamp,
            b'D' => FieldValueKind::Decimal,
            b'S' => FieldValueKind::LongStr,
            b'A' => FieldValueKind::FieldArray,
            b'F' => FieldValueKind::FieldTable,
            b'x' => FieldValueKind::BytesArray,
            b'V' => FieldValueKind::Void,
            _ => FieldValueKind::Unknown
        }
    }
}

#[derive(Debug)]
pub enum FieldValue {
    Boolean(bool),
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    F32(f32),
    F64(f64),
    Timestamp(Timestamp),
    Decimal(Decimal),
    LongStr(LongStr),
    FieldArray(FieldArray),
    FieldTable(FieldTable),
    BytesArray(LongStr),
    Void
}

impl FieldValue {
    #[inline]
    pub fn from_bool(value: bool) -> FieldValue {
        FieldValue::Boolean(value)
    }

    #[inline]
    pub fn from_u8(value: u8) -> FieldValue {
        FieldValue::U8(value)
    }

    #[inline]
    pub fn from_i8(value: i8) -> FieldValue {
        FieldValue::I8(value)
    }

    #[inline]
    pub fn from_i16(value: i16) -> FieldValue {
        FieldValue::I16(value)
    }

    #[inline]
    pub fn from_u16(value: u16) -> FieldValue {
        FieldValue::U16(value)
    }

    #[inline]
    pub fn from_i32(value: i32) -> FieldValue {
        FieldValue::I32(value)
    }

    #[inline]
    pub fn from_u32(value: u32) -> FieldValue {
        FieldValue::U32(value)
    }

    #[inline]
    pub fn from_i64(value: i64) -> FieldValue {
        FieldValue::I64(value)
    }

    #[inline]
    pub fn from_u64(value: u64) -> FieldValue {
        FieldValue::U64(value)
    }

    #[inline]
    pub fn from_f32(value: f32) -> FieldValue {
        FieldValue::F32(value)
    }

    #[inline]
    pub fn from_f64(value: f64) -> FieldValue {
        FieldValue::F64(value)
    }

    #[inline]
    pub fn from_timestamp(value: Timestamp) -> FieldValue {
        FieldValue::Timestamp(value)
    }

    #[inline]
    pub fn from_decimal(value: Decimal) -> FieldValue {
        FieldValue::Decimal(value)
    }

    #[inline]
    pub fn from_long_string(value: LongStr) -> FieldValue {
        FieldValue::LongStr(value)
    }

    #[inline]
    pub fn from_field_array(value: Vec<FieldValue>) ->FieldValue {
        FieldValue::FieldArray(value)
    }

    #[inline]
    pub fn from_field_table(value: FieldTable) ->FieldValue {
        FieldValue::FieldTable(value)
    }

    #[inline]
    pub fn from_bytes_array(value: BytesArray) ->FieldValue {
        FieldValue::BytesArray(value)
    }

    #[inline]
    pub fn from_void() ->FieldValue {
        FieldValue::Void
    }

    #[inline]
    fn get_value_kind(&self) -> FieldValueKind {
        match self {
            FieldValue::Boolean(_) => FieldValueKind::Boolean,
            FieldValue::U8(_) => FieldValueKind::U8,
            FieldValue::I8(_) => FieldValueKind::I8,
            FieldValue::U16(_) => FieldValueKind::U16,
            FieldValue::I16(_) => FieldValueKind::I16,
            FieldValue::U32(_) => FieldValueKind::U32,
            FieldValue::I32(_) => FieldValueKind::I32,
            FieldValue::U64(_) => FieldValueKind::U64,
            FieldValue::I64(_) => FieldValueKind::I64,
            FieldValue::F32(_) => FieldValueKind::F32,
            FieldValue::F64(_) => FieldValueKind::F64,
            FieldValue::Timestamp(_) => FieldValueKind::Timestamp,
            FieldValue::Decimal(_) => FieldValueKind::Decimal,
            FieldValue::LongStr(_) => FieldValueKind::LongStr,
            FieldValue::FieldArray(_) => FieldValueKind::FieldArray,
            FieldValue::FieldTable(_) => FieldValueKind::FieldTable,
            FieldValue::BytesArray(_) => FieldValueKind::BytesArray,
            FieldValue::Void => FieldValueKind::Void
        }
    }
}

impl Encode for FieldValue {
    /// Encode value by type.
    ///
    /// # Examples
    /// ```rust
    /// use amqp_proto::{FieldValue, Decimal, LongStr, FieldArray, FieldTable, FieldName};
    /// use bytes::{BytesMut, BufMut};
    /// use amqp_proto::codec::Encode;
    ///
    /// let v1 = FieldValue::from_bool(false);
    /// let mut buffer = BytesMut::with_capacity(128);
    /// v1.encode(&mut buffer);
    /// assert_eq!(&buffer[..], &[b't', 0]);
    ///
    /// let v2 = FieldValue::from_u8(12u8);
    /// buffer.clear();
    /// v2.encode(&mut buffer);
    /// assert_eq!(&buffer[..], &[b'B', 12u8]);
    ///
    /// let v3 = FieldValue::from_i8(12i8);
    /// buffer.clear();
    /// v3.encode(&mut buffer);
    /// assert_eq!(&buffer[..], &[b'b', 12u8]);
    ///
    /// let v4 = FieldValue::from_i16(0x1234i16);
    /// buffer.clear();
    /// v4.encode(&mut buffer);
    /// assert_eq!(&buffer[..], &[b's', 0x12u8, 0x34u8]);
    ///
    /// let v5 = FieldValue::from_u16(0x1234u16);
    /// buffer.clear();
    /// v5.encode(&mut buffer);
    /// assert_eq!(&buffer[..], &[b'u', 0x12u8, 0x34u8]);
    ///
    /// let v6 = FieldValue::from_u32(0x12345678u32);
    /// buffer.clear();
    /// v6.encode(&mut buffer);
    /// assert_eq!(&buffer[..], &[b'i', 0x12u8, 0x34u8, 0x56u8, 0x78u8]);
    ///
    /// let v7 = FieldValue::from_i32(0x12345678i32);
    /// buffer.clear();
    /// v7.encode(&mut buffer);
    /// assert_eq!(&buffer[..], &[b'I', 0x12u8, 0x34u8, 0x56u8, 0x78u8]);
    ///
    /// let v8 = FieldValue::from_u64(0x12345678u64);
    /// buffer.clear();
    /// v8.encode(&mut buffer);
    /// assert_eq!(&buffer[..], &[b'L', 0u8, 0, 0, 0, 0x12u8, 0x34u8, 0x56u8, 0x78u8]);
    ///
    /// let v9 = FieldValue::from_i64(0x12345678i64);
    /// buffer.clear();
    /// v9.encode(&mut buffer);
    /// assert_eq!(&buffer[..], &[b'l', 0u8, 0, 0, 0, 0x12u8, 0x34u8, 0x56u8, 0x78u8]);
    ///
    /// let v10 = FieldValue::from_f32(123.456f32);
    /// buffer.clear();
    /// v10.encode(&mut buffer);
    /// let mut tmp = BytesMut::with_capacity(64);
    /// tmp.put_u8(b'f');
    /// tmp.put_u32(123.456f32.to_bits());
    /// assert_eq!(&buffer[..], &tmp[..]);
    ///
    /// let v11 = FieldValue::from_f64(123.456f64);
    /// buffer.clear();
    /// v11.encode(&mut buffer);
    /// let mut tmp = BytesMut::with_capacity(64);
    /// tmp.put_u8(b'd');
    /// tmp.put_u64(123.456f64.to_bits());
    /// assert_eq!(&buffer[..], &tmp[..]);
    ///
    /// let v12 = FieldValue::from_timestamp(0x12345678u64);
    /// buffer.clear();
    /// v12.encode(&mut buffer);
    /// assert_eq!(&buffer[..], &[b'T', 0u8, 0, 0, 0, 0x12u8, 0x34u8, 0x56u8, 0x78u8]);
    ///
    /// let v13 = FieldValue::from_decimal(Decimal::new(2u8, 0x12345678u32));
    /// buffer.clear();
    /// v13.encode(&mut buffer);
    /// assert_eq!(&buffer[..], &[b'D', 0x2u8, 0x12, 0x34, 0x56, 0x78]);
    ///
    /// let v14 = FieldValue::from_long_string(LongStr::with_bytes(b"hello").unwrap());
    /// buffer.clear();
    /// v14.encode(&mut buffer);
    /// assert_eq!(&buffer[..], &[b'S', 0, 0, 0, 0x5u8, b'h', b'e', b'l', b'l', b'o']);
    ///
    /// let mut arr = FieldArray::new();
    /// arr.push(FieldValue::from_u8(0x1));
    /// arr.push(FieldValue::from_u8(0x2));
    /// arr.push(FieldValue::from_u8(0x3));
    /// let v15 = FieldValue::from_field_array(arr);
    /// buffer.clear();
    /// v15.encode(&mut buffer);
    /// assert_eq!(&buffer[..], &[b'A', 0x0u8, 0, 0, 0x6u8, b'B', 0x1, b'B', 0x2, b'B', 0x3]);
    ///
    /// let mut table = FieldTable::new();
    /// table.insert(FieldName::with_bytes(b"hello").unwrap(), FieldValue::from_u32(0x12345678u32));
    /// table.insert(FieldName::with_bytes(b"world").unwrap(), FieldValue::from_long_string(LongStr::with_bytes(b"hello").unwrap()));
    /// let mut ret = BytesMut::with_capacity(128);
    /// ret.put_u8(b'F');
    /// ret.put_u32(27u32);
    /// for (k, _) in &table {
    ///     if *k == FieldName::with_bytes(b"hello").unwrap() {
    ///         ret.put_u8(5u8);
    ///         ret.put_slice(b"hello");
    ///         ret.put_u8(b'i');
    ///         ret.put_u32(0x12345678u32);
    ///     } else {
    ///         ret.put_u8(5u8);
    ///         ret.put_slice(b"world");
    ///         ret.put_u8(b'S');
    ///         ret.put_u32(5u32);
    ///         ret.put_slice(b"hello");
    ///     }
    /// }
    /// let value = FieldValue::from_field_table(table);
    /// buffer.clear();
    /// value.encode(&mut buffer);
    /// assert_eq!(&buffer[..], &ret[..]);
    ///
    /// let value = FieldValue::from_bytes_array(LongStr::with_bytes(b"hello").unwrap());
    /// let mut ret = BytesMut::with_capacity(8);
    /// ret.put_u8(b'x');
    /// ret.put_u32(0x5u32);
    /// ret.put_slice(b"hello");
    /// buffer.clear();
    /// value.encode(&mut buffer);
    /// assert_eq!(&buffer[..], &ret[..]);
    ///
    /// let value = FieldValue::from_void();
    /// let ret = [b'V'];
    /// buffer.clear();
    /// value.encode(&mut buffer);
    /// assert_eq!(&buffer[..], ret)
    /// ```
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.put_u8(self.get_value_kind().as_u8());
        match self {
            FieldValue::Boolean(v) => {
                let v: u8 = if *v { 1 } else { 0 };
                buffer.put_u8(v);
            }
            FieldValue::U8(v) => buffer.put_u8(*v),
            FieldValue::I8(v) => buffer.put_i8(*v),
            FieldValue::U16(v) => buffer.put_u16(*v),
            FieldValue::I16(v) => buffer.put_i16(*v),
            FieldValue::U32(v) => buffer.put_u32(*v),
            FieldValue::I32(v) => buffer.put_i32(*v),
            FieldValue::U64(v) => buffer.put_u64(*v),
            FieldValue::I64(v) => buffer.put_i64(*v),
            FieldValue::F32(v) => buffer.put_f32(*v),
            FieldValue::F64(v) => buffer.put_f64(*v),
            FieldValue::Timestamp(v) => buffer.put_u64(*v),
            FieldValue::Decimal(v) => v.encode(buffer),
            FieldValue::LongStr(v) => v.encode(buffer),
            FieldValue::FieldArray(v) => {
                v.encode(buffer);
            }
            FieldValue::FieldTable(v) => {
                v.encode(buffer);
            }
            FieldValue::BytesArray(v) => {
                v.encode(buffer);
            }
            FieldValue::Void => {}
        }
    }
}

impl Decode<FieldValue> for FieldValue {
    /// Decode FieldValue from bytes
    ///
    /// # Examples
    /// ```rust
    /// use amqp_proto::{FieldValue, FieldName, FieldTable};
    /// use amqp_proto::Decimal;
    /// use amqp_proto::LongStr;
    /// use amqp_proto::codec::Decode;
    /// use bytes::{BytesMut, BufMut};
    ///
    /// let b1 = [b't', 0u8];
    /// let (_, v1) = FieldValue::decode(&b1).unwrap();
    /// assert!(matches!(v1, FieldValue::Boolean(v) if !v));
    ///
    /// let b2 = [b'B', 0x1u8];
    /// let (_, v2) = FieldValue::decode(&b2).unwrap();
    /// assert!(matches!(v2, FieldValue::U8(v) if v == 0x1u8));
    ///
    /// let b3 = [b'b', 0x1u8];
    /// let (_, v3) = FieldValue::decode(&b3).unwrap();
    /// assert!(matches!(v3, FieldValue::I8(v) if v == 0x1i8));
    ///
    /// let b4 = [b's', 0x12u8, 0x34u8];
    /// let (_, v4) = FieldValue::decode(&b4).unwrap();
    /// assert!(matches!(v4, FieldValue::I16(v) if v == 0x1234i16));
    ///
    /// let b5 = [b'u', 0x12u8, 0x34u8];
    /// let (_, v5) = FieldValue::decode(&b5).unwrap();
    /// assert!(matches!(v5, FieldValue::U16(v) if v == 0x1234u16));
    ///
    /// let b6 = [b'I', 0x12u8, 0x34u8, 0x56u8, 0x78u8];
    /// let (_, v6) = FieldValue::decode(&b6).unwrap();
    /// assert!(matches!(v6, FieldValue::I32(v) if v == 0x12345678i32));
    ///
    /// let b7 = [b'i', 0x12u8, 0x34u8, 0x56u8, 0x78u8];
    /// let (_, v7) = FieldValue::decode(&b7).unwrap();
    /// assert!(matches!(v7, FieldValue::U32(v) if v == 0x12345678u32));
    ///
    /// let b8 = [b'l', 0x0u8, 0, 0, 0, 0x12u8, 0x34u8, 0x56u8, 0x78u8];
    /// let (_, v8) = FieldValue::decode(&b8).unwrap();
    /// assert!(matches!(v8, FieldValue::I64(v) if v == 0x12345678i64));
    ///
    /// let b9 = [b'L',  0x0u8, 0, 0, 0, 0x12u8, 0x34u8, 0x56u8, 0x78u8];
    /// let (_, v9) = FieldValue::decode(&b9).unwrap();
    /// assert!(matches!(v9, FieldValue::U64(v) if v == 0x12345678u64));
    ///
    /// let b10 = [b'T',  0x0u8, 0, 0, 0, 0x12u8, 0x34u8, 0x56u8, 0x78u8];
    /// let (_, v10) = FieldValue::decode(&b10).unwrap();
    /// assert!(matches!(v10, FieldValue::Timestamp(v) if v == 0x12345678u64));
    ///
    /// let mut b11 = BytesMut::with_capacity(64);
    /// b11.put_u8(b'f');
    /// b11.put_u32(123.456f32.to_bits());
    /// let (_, v11) = FieldValue::decode(&b11).unwrap();
    /// assert!(matches!(v11, FieldValue::F32(v) if v.to_bits() == 123.456f32.to_bits()));
    ///
    /// let mut b12 = BytesMut::with_capacity(64);
    /// b12.put_u8(b'd');
    /// b12.put_u64(123.456f64.to_bits());
    /// let (_, v12) = FieldValue::decode(&b12).unwrap();
    /// assert!(matches!(v12, FieldValue::F64(v) if v.to_bits() == 123.456f64.to_bits()));
    ///
    /// let b13 = [b'D', 0x2, 0, 0, 0, 0x12];
    /// let (_, v13) = FieldValue::decode(&b13).unwrap();
    /// assert!(matches!(v13, FieldValue::Decimal(v) if v == Decimal::new(2, 0x12u32)));
    ///
    /// let b14 = [b'S', 0, 0, 0, 5u8, b'h', b'e', b'l', b'l', b'o'];
    /// let (_, v14) = FieldValue::decode(&b14).unwrap();
    /// assert!(matches!(v14, FieldValue::LongStr(v) if v.to_string() == String::from("hello")));
    ///
    /// let (_, arr) = FieldValue::decode(&[b'A', 0x0u8, 0, 0, 14u8, b'B', 0x1, b'B', 0x2, b'S', 0, 0, 0, 5u8, 104, 101, 108, 108, 111]).unwrap();
    /// match arr {
    ///     FieldValue::FieldArray(arr) => {
    ///         assert!(matches!(arr[0], FieldValue::U8(v) if v == 0x1u8));
    ///         assert!(matches!(arr[1], FieldValue::U8(v) if v == 0x2u8));
    ///         assert!(matches!(arr[2], FieldValue::LongStr(ref v) if v.to_string() == String::from("hello")));
    ///     }
    ///     _ => panic!("Should be FieldArray")
    /// }
    ///
    /// let (_, arr) = FieldValue::decode(&[b'x', 0, 0, 0, 5u8, 104, 101, 108, 108, 111]).unwrap();
    /// assert!(matches!(arr, FieldValue::BytesArray(v) if v.to_string() == String::from("hello")));
    ///
    /// let mut table = FieldTable::new();
    /// table.insert(FieldName::with_bytes(b"hello").unwrap(), FieldValue::from_u32(0x12345678u32));
    /// table.insert(FieldName::with_bytes(b"world").unwrap(), FieldValue::from_long_string(LongStr::with_bytes(b"hello").unwrap()));
    /// let mut ret = BytesMut::with_capacity(128);
    /// ret.put_u8(b'F');
    /// ret.put_u32(27u32);
    /// for (k, _) in &table {
    ///     if *k == FieldName::with_bytes(b"hello").unwrap() {
    ///         ret.put_u8(5u8);
    ///         ret.put_slice(b"hello");
    ///         ret.put_u8(b'i');
    ///         ret.put_u32(0x12345678u32);
    ///     } else {
    ///         ret.put_u8(5u8);
    ///         ret.put_slice(b"world");
    ///         ret.put_u8(b'S');
    ///         ret.put_u32(5u32);
    ///         ret.put_slice(b"hello");
    ///     }
    /// }
    /// if let (_, FieldValue::FieldTable(t)) = FieldValue::decode(&ret).unwrap() {
    ///     assert!(matches!(t.get(&FieldName::with_bytes(b"hello").unwrap()).unwrap(), FieldValue::U32(v) if *v == 0x12345678u32));
    ///     assert!(matches!(t.get(&FieldName::with_bytes(b"world").unwrap()).unwrap(), FieldValue::LongStr(v) if v.to_string() == String::from("hello")));
    /// } else {
    ///     panic!("Expected FieldTable value");
    /// }
    ///
    /// let (_, v) = FieldValue::decode(&[b'V']).unwrap();
    /// assert!(matches!(v, FieldValue::Void));
    /// ```
    fn decode(buffer: &[u8]) -> Result<(&[u8], FieldValue), FrameDecodeErr> {
        let (buffer, value_type) = match u8::decode(buffer) {
            Ok(v) => v,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode FieldValue type -> {}", e)))
        };
        match FieldValueKind::from(value_type) {
            FieldValueKind::Boolean => {
                match u8::decode(buffer) {
                    Ok((buffer, value)) => {
                        if value == 0u8 {
                            Ok((buffer, FieldValue::from_bool(false)))
                        } else {
                            Ok((buffer, FieldValue::from_bool(true)))
                        }
                    },
                    Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode FieldValue boolean -> {}", e)))
                }
            }
            FieldValueKind::I8 => i8::decode(buffer).map_err(|e| FrameDecodeErr::DecodeError(format!("decode FieldValue i8 -> {}", e))).map(|(buffer, v)|(buffer, FieldValue::from_i8(v))),
            FieldValueKind::U8 => u8::decode(buffer).map_err(|e| FrameDecodeErr::DecodeError(format!("decode FieldValue u8 -> {}", e))).map(|(buffer, v)|(buffer, FieldValue::from_u8(v))),
            FieldValueKind::I16 => i16::decode(buffer).map_err(|e| FrameDecodeErr::DecodeError(format!("decode FieldValue i16 -> {}", e))).map(|(buffer, v)|(buffer, FieldValue::from_i16(v))),
            FieldValueKind::U16 => u16::decode(buffer).map_err(|e| FrameDecodeErr::DecodeError(format!("decode FieldValue u16 -> {}", e))).map(|(buffer, v)|(buffer, FieldValue::from_u16(v))),
            FieldValueKind::I32 => i32::decode(buffer).map_err(|e| FrameDecodeErr::DecodeError(format!("decode FieldValue i32 -> {}", e))).map(|(buffer, v)|(buffer, FieldValue::from_i32(v))),
            FieldValueKind::U32 => u32::decode(buffer).map_err(|e| FrameDecodeErr::DecodeError(format!("decode FieldValue u32 -> {}", e))).map(|(buffer, v)|(buffer, FieldValue::from_u32(v))),
            FieldValueKind::I64 => i64::decode(buffer).map_err(|e| FrameDecodeErr::DecodeError(format!("decode FieldValue i64 -> {}", e))).map(|(buffer, v)|(buffer, FieldValue::from_i64(v))),
            FieldValueKind::U64 => u64::decode(buffer).map_err(|e| FrameDecodeErr::DecodeError(format!("decode FieldValue u64 -> {}", e))).map(|(buffer, v)|(buffer, FieldValue::from_u64(v))),
            FieldValueKind::F32 => f32::decode(buffer).map_err(|e| FrameDecodeErr::DecodeError(format!("decode FieldValue f32 -> {}", e))).map(|(buffer, v)|(buffer, FieldValue::from_f32(v))),
            FieldValueKind::F64 => f64::decode(buffer).map_err(|e| FrameDecodeErr::DecodeError(format!("decode FieldValue f64 -> {}", e))).map(|(buffer, v)|(buffer, FieldValue::from_f64(v))),
            FieldValueKind::Timestamp => u64::decode(buffer).map_err(|e| FrameDecodeErr::DecodeError(format!("decode FieldValue timestamp -> {}", e))).map(|(buffer, v)|(buffer, FieldValue::from_timestamp(v))),
            FieldValueKind::Decimal => Decimal::decode(buffer).map_err(|e| FrameDecodeErr::DecodeError(format!("decode FieldValue decimal -> {}", e))).map(|(buffer, v)|(buffer, FieldValue::from_decimal(v))),
            FieldValueKind::LongStr => LongStr::decode(buffer).map_err(|e| FrameDecodeErr::DecodeError(format!("decode FieldValue long string -> {}", e))).map(|(buffer, v)|(buffer, FieldValue::from_long_string(v))),
            FieldValueKind::FieldArray => FieldArray::decode(buffer).map_err(|e| FrameDecodeErr::DecodeError(format!("decode FieldValue FieldArray -> {}", e))).map(|(buffer, v)|(buffer, FieldValue::from_field_array(v))),
            FieldValueKind::BytesArray => ByteArray::decode(buffer).map_err(|e| FrameDecodeErr::DecodeError(format!("decode FieldValue ByteArray -> {}", e))).map(|(buffer, v)|(buffer, FieldValue::from_bytes_array(v))),
            FieldValueKind::FieldTable => FieldTable::decode(buffer).map_err(|e| FrameDecodeErr::DecodeError(format!("decode FieldValue FieldTable -> {}", e))).map(|(buffer, v)|(buffer, FieldValue::from_field_table(v))),
            FieldValueKind::Void => Ok((buffer, FieldValue::from_void())),
            FieldValueKind::Unknown => return Err(FrameDecodeErr::DecodeError(format!("decode FieldValue failed, unknown field value kind")))
        }
    }
}

pub type FieldTable = HashMap<FieldName, FieldValue>;

impl Encode for FieldTable {
    /// Encode FieldTable to BytesMut
    ///
    /// # Examples
    /// ```rust
    /// use amqp_proto::{FieldTable, FieldName, FieldValue, LongStr};
    /// use bytes::{BytesMut, BufMut};
    /// use amqp_proto::codec::Encode;
    ///
    /// let mut table = FieldTable::new();
    /// table.insert(FieldName::with_bytes(b"hello").unwrap(), FieldValue::from_u32(0x12345678u32));
    /// table.insert(FieldName::with_bytes(b"world").unwrap(), FieldValue::from_long_string(LongStr::with_bytes(b"hello").unwrap()));
    /// let mut buffer = BytesMut::with_capacity(64);
    /// table.encode(&mut buffer);
    ///
    /// let mut ret = BytesMut::with_capacity(128);
    /// ret.put_u32(27u32);
    /// for (k, _) in &table {
    ///     if *k == FieldName::with_bytes(b"hello").unwrap() {
    ///         ret.put_u8(5u8);
    ///         ret.put_slice(b"hello");
    ///         ret.put_u8(b'i');
    ///         ret.put_u32(0x12345678u32);
    ///     } else {
    ///         ret.put_u8(5u8);
    ///         ret.put_slice(b"world");
    ///         ret.put_u8(b'S');
    ///         ret.put_u32(5u32);
    ///         ret.put_slice(b"hello");
    ///     }
    /// }
    /// assert_eq!(&buffer[..], &ret[..]);
    /// ```
    #[inline]
    fn encode(&self, buffer: &mut BytesMut) {
        let mut index = buffer.len();
        buffer.put_u32(0);
        for (k, v) in self {
            k.encode(buffer);
            v.encode(buffer);
        }
        let field_table_len = (buffer.len() - index - std::mem::size_of::<u32>()) as u32;
        // set the true length of the field table
        for i in &field_table_len.to_be_bytes() {
            buffer[index] = *i;
            index += 1;
        }
    }
}

impl Decode<FieldTable> for FieldTable {
    /// Decode FieldTable from bytes
    ///
    /// # Examples
    /// ```rust
    /// use amqp_proto::{FieldTable, FieldName, FieldValue, LongStr};
    /// use bytes::{BytesMut, BufMut};
    /// use amqp_proto::codec::Decode;
    /// let mut table = FieldTable::new();
    /// table.insert(FieldName::with_bytes(b"hello").unwrap(), FieldValue::from_u32(0x12345678u32));
    /// table.insert(FieldName::with_bytes(b"world").unwrap(), FieldValue::from_long_string(LongStr::with_bytes(b"hello").unwrap()));
    /// let mut ret = BytesMut::with_capacity(128);
    /// ret.put_u32(27u32);
    /// for (k, _) in &table {
    ///     if *k == FieldName::with_bytes(b"hello").unwrap() {
    ///         ret.put_u8(5u8);
    ///         ret.put_slice(b"hello");
    ///         ret.put_u8(b'i');
    ///         ret.put_u32(0x12345678u32);
    ///     } else {
    ///         ret.put_u8(5u8);
    ///         ret.put_slice(b"world");
    ///         ret.put_u8(b'S');
    ///         ret.put_u32(5u32);
    ///         ret.put_slice(b"hello");
    ///     }
    /// }
    /// let (_, t) = FieldTable::decode(&ret).unwrap();
    /// assert!(matches!(t.get(&FieldName::with_bytes(b"hello").unwrap()).unwrap(), FieldValue::U32(v) if *v == 0x12345678u32));
    /// assert!(matches!(t.get(&FieldName::with_bytes(b"world").unwrap()).unwrap(), FieldValue::LongStr(v) if v.to_string() == String::from("hello")));
    /// ```
    fn decode(buffer: &[u8]) -> Result<(&[u8], FieldTable), FrameDecodeErr> {
        let (buffer, length) = match u32::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode FieldTable length -> {}", e)))
        };
        let (buffer, data) = match take_bytes(buffer, length as usize) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode FieldTable bytes -> {}", e)))
        };

        let mut table = FieldTable::new();
        let mut tmp = data;
        loop {
            let (retain, name) = match FieldName::decode(tmp) {
                Ok(ret) => ret,
                Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode FieldTable FieldName failed: {}", e)))
            };
            let (retain, value) = match FieldValue::decode(retain) {
                Ok(ret) => ret,
                Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode FieldTable FieldValue failed: {}", e)))
            };
            tmp = retain;
            table.insert(name, value);
            if tmp.len() == 0 {
                return Ok((buffer, table))
            }
        }
    }
}




/// frame end octet, every frame should be end with 0xce
pub const FRAME_END: u8 = 0xce;

/// frame type, amqp protocol contains METHOD, HEARTBEAT, HEADER, CONTENT BODY frame
pub enum FrameType {
    METHOD,
    HEADER,
    BODY,
    HEARTBEAT,
    UNKNOWN
}

impl FrameType {
    #[inline]
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
    #[inline]
    fn default() -> Self {
        FrameType::METHOD
    }
}

impl From<u8> for FrameType {
    #[inline]
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

/// While tcp connection is established, the client should send protocol header to server
#[derive(Property)]
#[property(get(public), set(public))]
pub struct ProtocolHeader {
    protocol: Vec<u8>,
    major_id: u8,
    minor_id: u8,
    major_version: u8,
    minor_version: u8
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

impl Encode for ProtocolHeader {
    /// Encode ProtocolHeader to BytesMut
    ///
    /// # Examples
    /// ```rust
    /// use amqp_proto::codec::{ProtocolHeader, Encode};
    /// use bytes::BytesMut;
    ///
    /// let protocol_header = ProtocolHeader::default();
    /// let buf = [0x41u8, 0x4d, 0x51, 0x50, 0, 0, 9, 1];
    /// let mut buffer = BytesMut::with_capacity(16);
    /// protocol_header.encode(&mut buffer);
    /// assert_eq!(&buf[..], &buffer[..]);
    /// ```
    #[inline]
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.extend_from_slice(&self.protocol);
        buffer.put_u8(self.major_id);
        buffer.put_u8(self.minor_id);
        buffer.put_u8(self.major_version);
        buffer.put_u8(self.minor_version);
    }
}

impl Decode<ProtocolHeader> for ProtocolHeader {
    /// Decode ProtocolHeader from bytes
    ///
    /// # Examples
    /// ```rust
    /// use amqp_proto::codec::{ProtocolHeader, Decode};
    ///
    /// let buf = [0x41u8, 0x4d, 0x51, 0x50, 0, 0, 9, 1];
    /// let (_, header) = ProtocolHeader::decode(&buf).unwrap();
    /// assert_eq!(&header.protocol(), b"AMQP");
    /// assert_eq!(header.major_id(), 0u8);
    /// assert_eq!(header.minor_id(), 0u8);
    /// assert_eq!(header.major_version(), 9u8);
    /// assert_eq!(header.minor_version(), 1u8);
    /// ```
    fn decode(buffer: &[u8]) -> Result<(&[u8], ProtocolHeader), FrameDecodeErr> {
        let (buffer, protocol) = match take_bytes(buffer, b"AMQP".len()) {
            Ok((buffer, protocol)) => {
                if b"AMQP" != protocol {
                    return Err(FrameDecodeErr::SyntaxError("Wrong protocol, expected AMQP"))
                } else { (buffer, protocol) }
            }
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode protocol scheme -> {}", e)))
        };
        let (buffer, major_id) = match u8::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode major_id -> {}", e)))
        };
        let (buffer, minor_id) = match u8::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode minor_id -> {}", e)))
        };
        let (buffer, major_version) = match u8::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode major_version -> {}", e)))
        };
        let (buffer, minor_version) = match u8::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode minor_version -> {}", e)))
        };
        Ok((buffer, ProtocolHeader { protocol: Vec::from(protocol), major_id, minor_id, major_version, minor_version }))
    }
}

/// This is Content Header Frame  properties
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
    #[inline]
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
    #[inline]
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
    #[inline]
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.put_u16(self.class.class_id());
        buffer.put_u16(self.method.method_id());
        self.args.encode(buffer);
    }
}

impl Decode<MethodPayload> for MethodPayload {
    fn decode(buffer: &[u8]) -> Result<(&[u8], MethodPayload), FrameDecodeErr>{
        let (buffer, class_id) = match u16::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => {
                match e {
                    FrameDecodeErr::Incomplete => return Err(e),
                    _ => return Err(FrameDecodeErr::DecodeError(format!("decode MethodPayload class id failed -> {}", e)))
                }
            }
        };
        let (buffer, method_id) = match u16::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => {
                match e {
                    FrameDecodeErr::Incomplete => return Err(e),
                    _ => return Err(FrameDecodeErr::DecodeError(format!("decode MethodPayload method id failed -> {}", e)))
                }
            }
        };
        let class = Class::from(class_id);
        if let Class::Unknown = class { 
            return Err(FrameDecodeErr::DecodeError(format!("decode MethodPayload unknown class: {}", class_id)));
        }
        let method = match get_method_type(class, method_id) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode MethodPayload unknown method -> {}", e)))
        };
        let arguments = match method {
            Method::Connection(method_type) => {
                match method_type {
                    ConnectionMethod::Start => ConnectionStart::decode(buffer),
                    ConnectionMethod::StartOk => ConnectionStartOk::decode(buffer),
                    ConnectionMethod::Tune => ConnectionTune::decode(buffer),
                    ConnectionMethod::TuneOk => ConnectionTuneOk::decode(buffer),
                    ConnectionMethod::Secure => ConnectionSecure::decode(buffer),
                    ConnectionMethod::SecureOk => ConnectionSecureOk::decode(buffer),
                    ConnectionMethod::Open => ConnectionOpen::decode(buffer),
                    ConnectionMethod::OpenOk => ConnectionOpenOk::decode(buffer),
                    ConnectionMethod::Close => ConnectionClose::decode(buffer),
                    ConnectionMethod::CloseOk => ConnectionCloseOk::decode(buffer),
                    ConnectionMethod::Unknown => return Err(FrameDecodeErr::SyntaxError("decode MethodPayload unknown connection method"))
                }
            },
            Method::Channel(method_type) => {
                match method_type {
                    ChannelMethod::Open => ChannelOpen::decode(buffer),
                    ChannelMethod::OpenOk => ChannelOpenOk::decode(buffer),
                    ChannelMethod::Flow => ChannelFlow::decode(buffer),
                    ChannelMethod::FlowOk => ChannelFlowOk::decode(buffer),
                    ChannelMethod::Close => ChannelClose::decode(buffer),
                    ChannelMethod::CloseOk => ChannelCloseOk::decode(buffer),
                    ChannelMethod::Unknown => return Err(FrameDecodeErr::SyntaxError("decode MethodPayload unknown channel method"))
                }
            },
            Method::Access(method_type) => {
                match method_type {
                    AccessMethod::Request => AccessRequest::decode(buffer),
                    AccessMethod::RequestOk => AccessRequestOk::decode(buffer),
                    AccessMethod::Unknown => return Err(FrameDecodeErr::SyntaxError("decode MethodPayload unknown access method"))
                }
            },
            Method::Exchange(method_type) => {
                match method_type {
                    ExchangeMethod::Declare => ExchangeDeclare::decode(buffer),
                    ExchangeMethod::DeclareOk => ExchangeDeclareOk::decode(buffer),
                    ExchangeMethod::Bind => ExchangeBind::decode(buffer),
                    ExchangeMethod::BindOk => ExchangeBindOk::decode(buffer),
                    ExchangeMethod::Unbind => ExchangeUnbind::decode(buffer),
                    ExchangeMethod::UnbindOk => ExchangeUnbindOk::decode(buffer),
                    ExchangeMethod::Delete => ExchangeDelete::decode(buffer),
                    ExchangeMethod::DeleteOk => ExchangeDeleteOk::decode(buffer),
                    ExchangeMethod::Unknown => return Err(FrameDecodeErr::SyntaxError("decode MethodPayload unknown exchange method"))
                }
            },
            Method::Queue(method_type) => {
                match method_type {
                    QueueMethod::Declare => QueueDeclare::decode(buffer),
                    QueueMethod::DeclareOk => QueueDeclareOk::decode(buffer),
                    QueueMethod::Bind => QueueBind::decode(buffer),
                    QueueMethod::BindOk => QueueBindOk::decode(buffer),
                    QueueMethod::Unbind => QueueUnbind::decode(buffer),
                    QueueMethod::UnbindOk => QueueUnbindOk::decode(buffer),
                    QueueMethod::Purge => QueuePurge::decode(buffer),
                    QueueMethod::PurgeOk => QueuePurgeOk::decode(buffer),
                    QueueMethod::Delete => QueueDelete::decode(buffer),
                    QueueMethod::DeleteOk => QueueDeleteOk::decode(buffer),
                    QueueMethod::Unknown => return Err(FrameDecodeErr::SyntaxError("decode MethodPayload unknown queue method"))
                }
            },
            Method::Basic(method_type) => {
                match method_type {
                    BasicMethod::Qos => BasicQos::decode(buffer),
                    BasicMethod::QosOk => BasicQosOk::decode(buffer),
                    BasicMethod::Consume => BasicConsume::decode(buffer),
                    BasicMethod::ConsumeOk => BasicConsumeOk::decode(buffer),
                    BasicMethod::Cancel => BasicCancel::decode(buffer),
                    BasicMethod::CancelOk => BasicCancelOk::decode(buffer),
                    BasicMethod::Publish => BasicPublish::decode(buffer),
                    BasicMethod::Return => BasicReturn::decode(buffer),
                    BasicMethod::Deliver => BasicDeliver::decode(buffer),
                    BasicMethod::Get => BasicGet::decode(buffer),
                    BasicMethod::GetEmpty => BasicGetEmpty::decode(buffer),
                    BasicMethod::GetOk => BasicGetOk::decode(buffer),
                    BasicMethod::Reject => BasicReject::decode(buffer),
                    BasicMethod::RecoverAsync => BasicRecoverAsync::decode(buffer),
                    BasicMethod::Recover => BasicRecover::decode(buffer),
                    BasicMethod::RecoverOk => BasicRecoverOk::decode(buffer),
                    BasicMethod::Ack => BasicAck::decode(buffer),
                    BasicMethod::Nack => BasicNack::decode(buffer),
                    BasicMethod::Unknown => return Err(FrameDecodeErr::SyntaxError("decode MethodPayload unknown basic method"))
                }
            },
            Method::Tx(method_type) => {
                match method_type {
                    TxMethod::Select => TxSelect::decode(buffer),
                    TxMethod::SelectOk => TxSelectOk::decode(buffer),
                    TxMethod::Commit => TxCommit::decode(buffer),
                    TxMethod::CommitOk => TxCommitOk::decode(buffer),
                    TxMethod::Rollback => TxRollback::decode(buffer),
                    TxMethod::RollbackOk => TxRollbackOk::decode(buffer),
                    TxMethod::Unknown => return Err(FrameDecodeErr::SyntaxError("decode MethodPayload unknown tx method"))
                }
            },
            Method::Confirm(method_type) => {
                match method_type {
                    ConfirmMethod::Select => ConfirmSelect::decode(buffer),
                    ConfirmMethod::SelectOk => ConfirmSelectOk::decode(buffer),
                    ConfirmMethod::Unknown => return Err(FrameDecodeErr::SyntaxError("decode MethodPayload unknown confirm method"))
                }
            }
        };

        let (buffer, args) = match arguments {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode MethodPayload error: {}", e)))
        };
        Ok((buffer, MethodPayload { class, method, args}))
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
    #[inline]
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.put_u16(self.class.class_id());
        buffer.put_u16(self.weight);
        buffer.put_u64(self.body_size);
        self.properties.encode(buffer);
    }
}

impl Decode<ContentHeaderPayload> for ContentHeaderPayload {
    fn decode(buffer: &[u8]) -> Result<(&[u8], ContentHeaderPayload), FrameDecodeErr> {
        // pase payload
        let (buffer, class_id) = match u16::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode ContentHeaderPayload class id failed -> {}", e)))
        };
        let class_type = Class::from(class_id);
        if let Class::Unknown = class_type {
            return Err(FrameDecodeErr::DecodeError(format!("decode ContentHeaderPayload class tyep unknown -> {}", class_id)));
        }
        let (buffer, weight) = match u16::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode ContentHeaderPayload weight -> {}", e)))
        };
        let (buffer, body_size) = match u64::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode ContentHeaderPayload body_size -> {}", e)))
        };

        let properties = match class_type {
            Class::Connection => ConnectionProperties::decode(buffer),
            Class::Access => AccessProperties::decode(buffer),
            Class::Exchange => ExchangeProperties::decode(buffer),
            Class::Channel => ChannelProperties::decode(buffer),
            Class::Queue => QueueProperties::decode(buffer),
            Class::Basic => BasicProperties::decode(buffer),
            Class::Tx => TxProperties::decode(buffer),
            Class::Confirm => ConfirmProperties::decode(buffer),
            Class::Unknown => return Err(FrameDecodeErr::SyntaxError("decode ContentHeaderPayload unknown class"))
        };
        let (buffer, properties) = match properties {
            Ok(properties) => properties,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode ContentHeaderPayload properties -> {}", e)))
        };
        Ok((buffer, ContentHeaderPayload { class: class_type, weight, body_size, properties }))
    }
}

pub struct HeartbeatPayload;

impl Encode for HeartbeatPayload {
    #[inline]
    fn encode(&self, _: &mut BytesMut) {
    }
}

impl Decode<HeartbeatPayload> for HeartbeatPayload {
    #[inline]
    fn decode(buffer: &[u8]) -> Result<(&[u8], HeartbeatPayload), FrameDecodeErr>{
        Ok((buffer, HeartbeatPayload))
    }
}

pub enum Payload {
    Heartbeat(HeartbeatPayload),
    Method(MethodPayload),
    ContentHeader(ContentHeaderPayload),
    ContentBody(Vec<u8>)
}

impl Default for Payload {
    #[inline]
    fn default() -> Self {
        Payload::Method(MethodPayload::default())
    }
}

impl Encode for Payload {
    #[inline]
    fn encode(&self, buffer: &mut BytesMut) {
        match self {
            Payload::Heartbeat(heartbeat) => heartbeat.encode(buffer),
            Payload::Method(method) => method.encode(buffer),
            Payload::ContentHeader(content_header) => content_header.encode(buffer),
            Payload::ContentBody(content_body) => buffer.extend_from_slice(content_body.as_slice()),
        }
    }
}

// frame
#[derive(Property, Default)]
#[property(get(public))]
pub struct Frame {
    frame_type: FrameType,
    channel: u16,
    length: u32,
    payload: Payload,
}

impl Frame {
    /// The whole frame bytes length
    #[inline]
    pub fn len(&self) -> usize {
        (self.length + 8u32) as usize
    }
}

impl Encode for Frame {
    #[inline]
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.put_u8(self.frame_type.frame_type_id());
        buffer.put_u16(self.channel);
        buffer.put_u32(self.length);
        self.payload.encode(buffer);
        buffer.put_u8(FRAME_END);
    }
}

impl Decode<Frame> for Frame {
    fn decode(buffer: &[u8]) -> Result<(&[u8], Frame), FrameDecodeErr>{
        let (buffer, frame_type) = match u8::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => {
                match e {
                    FrameDecodeErr::Incomplete => return Err(FrameDecodeErr::Incomplete),
                    _ => return Err(FrameDecodeErr::DecodeError(format!("decode Frame frame_type -> {}", e)))
                }
            }
        };
        let (buffer, channel) = match u16::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => {
                match e {
                    FrameDecodeErr::Incomplete => return Err(FrameDecodeErr::Incomplete),
                    _ => return Err(FrameDecodeErr::DecodeError(format!("decode Frame channle id -> {}", e)))
                }
            }
        };
        let (buffer, length) = match u32::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => {
                match e {
                    FrameDecodeErr::Incomplete => return Err(FrameDecodeErr::Incomplete),
                    _ => return Err(FrameDecodeErr::DecodeError(format!("decode Frame payload length -> {}", e)))
                }
            }
        };
        // read payload
        let (buffer, payload_data) = match take_bytes(buffer, length as usize) {
            Ok(ret) => ret,
            Err(e) => {
                match e {
                    FrameDecodeErr::Incomplete => return Err(FrameDecodeErr::Incomplete),
                    _ => return Err(FrameDecodeErr::DecodeError(format!("decode Frame payload data -> {}", e)))
                }
            }
        };
        // read frame end
        let (buffer, _) = match u8::decode(buffer) {
            Ok((buffer, frame_end)) => {
                if FRAME_END == frame_end {
                    (buffer, frame_end)
                } else {
                    return Err(FrameDecodeErr::DecodeError(format!("decode Frame end error: {}", frame_end)))
                }
            },
            Err(e) => {
                match e {
                    FrameDecodeErr::Incomplete => return Err(FrameDecodeErr::Incomplete),
                    _ => return Err(FrameDecodeErr::DecodeError(format!("decode Frame end -> {}", e)))
                }
            }
        };
        let frame_type = FrameType::from(frame_type);
        match frame_type {
            FrameType::HEARTBEAT => {
                match HeartbeatPayload::decode(payload_data) {
                    Ok((_, heartbeat_payload)) => Ok((buffer, Frame { frame_type, channel, length, payload: Payload::Heartbeat(heartbeat_payload)})),
                    Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode Frame heartbeat payload failed -> {}", e)))
                }
            }
            FrameType::METHOD => {
                match MethodPayload::decode(payload_data) {
                    Ok((_, method_payload)) => Ok((buffer, Frame { frame_type, channel, length, payload: Payload::Method(method_payload)})),
                    Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode Frame heartbeat payload failed -> {}", e)))
                }
            }
            FrameType::HEADER => {
                match ContentHeaderPayload::decode(payload_data) {
                    Ok((_, content_header_payload)) => Ok((buffer, Frame { frame_type, channel, length, payload: Payload::ContentHeader(content_header_payload)})),
                    Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode Frame content header payload failed -> {}", e)))
                }
            }
            FrameType::BODY => {
                let mut payload = Vec::with_capacity(length as usize);
                payload.extend_from_slice(payload_data);
                Ok((buffer, Frame { frame_type, channel, length, payload: Payload::ContentBody(payload) }))
            }
            FrameType::UNKNOWN => return Err(FrameDecodeErr::DecodeError(format!("decode Frame unknown frame type: {}", frame_type.frame_type_id()))),
        }
    }
}

