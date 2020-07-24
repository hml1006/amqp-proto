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

impl ShortStr {
    // build a ShortStr from bytes
    #[inline]
    pub fn with_bytes(bytes: &[u8]) -> Result<Self, FrameDecodeErr>{
        if bytes.len() > std::u8::MAX as usize {
            return Err(FrameDecodeErr::SyntaxError("ShortStr too long"));
        }
        Ok(ShortStr(String::from_utf8_lossy(bytes).to_string()))
    }
}

impl Encode for ShortStr {
    #[inline]
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.put_u8(self.0.len() as u8);
        buffer.extend_from_slice(&self.0.as_bytes());
    }
}

impl Decode<ShortStr> for ShortStr {
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

#[derive(Debug, Default)]
pub struct LongStr(String);

impl LongStr {
    // build a LongStr from bytes, the length will be convert to big endian
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
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.put_u32(self.0.len() as u32);
        buffer.extend_from_slice(self.0.as_bytes());
    }
}

impl Decode<LongStr> for LongStr {
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

#[derive(Debug)]
pub struct Decimal {
    scale: u8,
    value: u32
}

impl Decimal {
    pub fn new( scale: u8, value: u32) -> Self {
        Decimal { scale: scale, value: value }
    }
}

impl Encode for Decimal {
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.put_u8(self.scale);
        buffer.put_u32(self.value);
    }
}

impl Decode<Decimal> for Decimal {
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
impl FieldName {
    #[inline]
    pub fn with_bytes(bytes: &[u8]) -> Result<FieldName, FrameDecodeErr> {
        // field name first letter should be '$'  '#' or letter
        let is_start_char_ok = match bytes[0] {
            b'$' | b'#' => true,
            b'a'..=b'z' | b'A'..=b'Z' => true,
            _ => false
        };

        if !is_start_char_ok {
            return Err(FrameDecodeErr::SyntaxError("FieldName start char error: "));
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
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl Encode for FieldName {
    fn encode(&self, buffer: &mut BytesMut) {
        self.0.encode(buffer);
    }
}

impl Decode<FieldName> for FieldName {
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
    fn decode(buffer: &[u8]) -> Result<(&[u8], FieldArray), FrameDecodeErr> {
        // array bytes length
        let (buffer, length) = match u32::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode FieldArray length -> {}", e)))
        };
        let (buffer, data) = match take_bytes(buffer, length as usize) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode FieldArray bytes->{}", e)))
        };
        let mut arr: Vec<FieldValue> = Vec::new();

        loop {
            let (data, value) = match FieldValue::decode(data) {
                Ok(ret) => ret,
                Err(e) => return Err(FrameDecodeErr::DecodeError(format!("read FieldArray item failed -> {}", e)))
            };
            arr.push(value);
            if data.len() == 0 {
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
    ByteArray,      // same as rabbitmq, len + bytes
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
            FieldValueKind::ByteArray => b'x',
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
            b'x' => FieldValueKind::ByteArray,
            b'V' => FieldValueKind::Void,
            _ => FieldValueKind::Unknown
        }
    }
}

#[derive(Debug)]
enum FieldValueInner {
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

#[derive(Debug)]
pub struct FieldValue {
    kind: FieldValueKind,
    value: FieldValueInner
}

impl FieldValue {
    #[inline]
    pub fn from_bool(value: bool) -> FieldValue {
        FieldValue { kind: FieldValueKind::Boolean, value: FieldValueInner::Boolean(value) }
    }

    #[inline]
    pub fn from_u8(value: u8) -> FieldValue {
        FieldValue { kind: FieldValueKind::U8, value: FieldValueInner::U8(value) }
    }

    #[inline]
    pub fn from_i8(value: i8) -> FieldValue {
        FieldValue { kind: FieldValueKind::I8, value: FieldValueInner::I8(value) }
    }

    #[inline]
    pub fn from_i16(value: i16) -> FieldValue {
        FieldValue { kind: FieldValueKind::I16, value: FieldValueInner::I16(value) }
    }

    #[inline]
    pub fn from_u16(value: u16) -> FieldValue {
        FieldValue { kind: FieldValueKind::U16, value: FieldValueInner::U16(value) }
    }

    #[inline]
    pub fn from_i32(value: i32) -> FieldValue {
        FieldValue { kind: FieldValueKind::I32, value: FieldValueInner::I32(value)}
    }

    #[inline]
    pub fn from_u32(value: u32) -> FieldValue {
        FieldValue { kind: FieldValueKind::U32, value: FieldValueInner::U32(value)}
    }

    #[inline]
    pub fn from_i64(value: i64) -> FieldValue {
        FieldValue { kind: FieldValueKind::I64, value: FieldValueInner::I64(value)}
    }

    #[inline]
    pub fn from_u64(value: u64) -> FieldValue {
        FieldValue { kind: FieldValueKind::U64, value: FieldValueInner::U64(value)}
    }

    #[inline]
    pub fn from_f32(value: f32) -> FieldValue {
        FieldValue { kind: FieldValueKind::F32, value: FieldValueInner::F32(value)}
    }

    #[inline]
    pub fn from_f64(value: f64) -> FieldValue {
        FieldValue { kind: FieldValueKind::F64, value: FieldValueInner::F64(value)}
    }

    #[inline]
    pub fn from_timestamp(value: Timestamp) -> FieldValue {
        FieldValue { kind: FieldValueKind::Timestamp, value: FieldValueInner::Timestamp(value)}
    }

    #[inline]
    pub fn from_decimal(value: Decimal) -> FieldValue {
        FieldValue { kind: FieldValueKind::Decimal, value: FieldValueInner::Decimal(value)}
    }

    #[inline]
    pub fn from_long_string(value: LongStr) -> FieldValue {
        FieldValue { kind: FieldValueKind::LongStr, value: FieldValueInner::LongStr(value)}
    }

    #[inline]
    pub fn from_field_array(value: Vec<FieldValue>) ->FieldValue {
        FieldValue { kind: FieldValueKind::FieldArray, value: FieldValueInner::FieldArray(value)}
    }

    #[inline]
    pub fn from_field_table(value: FieldTable) ->FieldValue {
        FieldValue { kind: FieldValueKind::FieldTable, value: FieldValueInner::FieldTable(value)}
    }

    #[inline]
    pub fn from_bytes_array(value: BytesArray) ->FieldValue {
        FieldValue { kind: FieldValueKind::ByteArray, value: FieldValueInner::BytesArray(value)}
    }

    #[inline]
    pub fn from_void() ->FieldValue {
        FieldValue { kind: FieldValueKind::Void, value: FieldValueInner::Void}
    }

    #[inline]
    pub fn get_kind(&self) -> &FieldValueKind {
        &self.kind
    }
}

impl Encode for FieldValue {
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.put_u8(self.kind.as_u8());
        match &self.value {
            FieldValueInner::Boolean(v) => {
                let v: u8 = if *v { 1 } else { 0 };
                buffer.put_u8(v);
            }
            FieldValueInner::U8(v) => buffer.put_u8(*v),
            FieldValueInner::I8(v) => buffer.put_i8(*v),
            FieldValueInner::U16(v) => buffer.put_u16(*v),
            FieldValueInner::I16(v) => buffer.put_i16(*v),
            FieldValueInner::U32(v) => buffer.put_u32(*v),
            FieldValueInner::I32(v) => buffer.put_i32(*v),
            FieldValueInner::U64(v) => buffer.put_u64(*v),
            FieldValueInner::I64(v) => buffer.put_i64(*v),
            FieldValueInner::F32(v) => buffer.put_f32(*v),
            FieldValueInner::F64(v) => buffer.put_f64(*v),
            FieldValueInner::Timestamp(v) => buffer.put_u64(*v),
            FieldValueInner::Decimal(v) => v.encode(buffer),
            FieldValueInner::LongStr(v) => v.encode(buffer),
            FieldValueInner::FieldArray(v) => {
                v.encode(buffer);
            }
            FieldValueInner::FieldTable(v) => {
                v.encode(buffer);
            }
            FieldValueInner::BytesArray(v) => {
                v.encode(buffer);
            }
            FieldValueInner::Void => {}
        }
    }
}

impl Decode<FieldValue> for FieldValue {
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
            FieldValueKind::ByteArray => ByteArray::decode(buffer).map_err(|e| FrameDecodeErr::DecodeError(format!("decode FieldValue ByteArray -> {}", e))).map(|(buffer, v)|(buffer, FieldValue::from_bytes_array(v))),
            FieldValueKind::FieldTable => FieldTable::decode(buffer).map_err(|e| FrameDecodeErr::DecodeError(format!("decode FieldValue FieldTable -> {}", e))).map(|(buffer, v)|(buffer, FieldValue::from_field_table(v))),
            FieldValueKind::Void => Ok((buffer, FieldValue::from_void())),
            FieldValueKind::Unknown => return Err(FrameDecodeErr::DecodeError(format!("decode FieldValue failed, unknown field value kind")))
        }
    }
}

pub type FieldTable = HashMap<FieldName, FieldValue>;

impl Encode for FieldTable {
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

        loop {
            let (data, name) = match FieldName::decode(data) {
                Ok(ret) => ret,
                Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode FieldTable FieldName failed: {}", e)))
            };
            let (data, value) = match FieldValue::decode(data) {
                Ok(ret) => ret,
                Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode FieldTable FieldValue failed: {}", e)))
            };
            table.insert(name, value);
            if data.len() == 0 {
                return Ok((buffer, table))
            }
        }
    }
}




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

impl Decode<ProtocolHeader> for ProtocolHeader {
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
    fn default() -> Self {
        Payload::Method(MethodPayload::default())
    }
}

impl Encode for Payload {
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
    pub fn len(&self) -> usize {
        (self.length + 8u32) as usize
    }
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

