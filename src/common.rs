use bytes::{BytesMut, BufMut};
use std::collections::HashMap;
use std::hash::{Hasher, Hash};
use std::vec::Vec;
use std::string::String;
use crate::error;
use crate::connection::ConnectionMethod;
use crate::channel::ChannelMethod;
use crate::access::AccessMethod;
use crate::exchange::ExchangeMethod;
use crate::queue::QueueMethod;
use crate::basic::BasicMethod;
use crate::confirm::ConfirmMethod;
use crate::tx::TxMethod;

// amqp0-9-1 field name length allowed is 128
const MAX_FIELD_NAME_LEN: usize = 128;
// max long string bytes length allowed
const MAX_LONG_STR_LEN: usize = 64 * 1024;


pub trait WriteToBuf {
    // write data to bytes buffer
    fn write_to_buf(&self, buffer: &mut BytesMut);
}

pub trait MethodId {
    fn method_id(&self) -> u16;
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
    pub fn with_bytes(bytes: &[u8]) -> Result<Self, error::AmqpError>{
        if bytes.len() > std::u8::MAX as usize {
            return Err(error::AmqpError::from(error::AmqpErrorKind::SyntaxError));
        }
        Ok(ShortStr(String::from_utf8_lossy(bytes).to_string()))
    }
}

impl WriteToBuf for ShortStr {
    #[inline]
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u8(self.0.len() as u8);
        buffer.extend_from_slice(&self.0.as_bytes());
    }
}

#[derive(Debug, Default)]
pub struct LongStr(String);

impl LongStr {
    // build a LongStr from bytes, the length will be convert to big endian
    #[inline]
    pub fn with_bytes(bytes: &[u8]) -> Result<LongStr, crate::error::AmqpError> {
        if bytes.len() > MAX_LONG_STR_LEN {
            Err(crate::error::AmqpError::from(error::AmqpErrorKind::SyntaxError))
        } else {
            Ok(LongStr(String::from_utf8_lossy(bytes).to_string()))
        }
    }
}

impl WriteToBuf for LongStr {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u32(self.0.len() as u32);
        buffer.extend_from_slice(self.0.as_bytes());
    }
}

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
pub struct Decimal {
    scale: u8,
    value: u32
}

impl Decimal {
    pub fn new( scale: u8, value: u32) -> Self {
        Decimal { scale: scale, value: value }
    }
}

impl WriteToBuf for Decimal {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u8(self.scale);
        buffer.put_u32(self.value);
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct FieldName(ShortStr);
impl FieldName {
    #[inline]
    pub fn with_bytes(bytes: &[u8]) -> Result<FieldName, error::AmqpError> {
        // field name first letter should be '$'  '#' or letter
        let is_start_char_ok = match bytes[0] {
            b'$' | b'#' => true,
            b'a'..=b'z' | b'A'..=b'Z' => true,
            _ => false
        };

        if !is_start_char_ok {
            return Err(error::AmqpError::from(error::AmqpErrorKind::SyntaxError));
        }

        // max field name length is 128
        if bytes.len() > MAX_FIELD_NAME_LEN {
            return Err(error::AmqpError::from(error::AmqpErrorKind::SyntaxError));
        }

        match ShortStr::with_bytes(bytes) {
            Ok(value) => Ok(FieldName(value)),
            Err(e) => Err(e)
        }
    }
}

impl WriteToBuf for FieldName {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        self.0.write_to_buf(buffer);
    }
}

impl Hash for FieldName {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

pub type FieldArray = Vec<FieldValue>;

impl WriteToBuf for FieldArray {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        let mut index = buffer.len();
        buffer.put_u32(0);
        for item in self {
            item.write_to_buf(buffer);
        }
        let field_table_len = (buffer.len() - index - std::mem::size_of::<u32>()) as u32;
        // set the true length of the field table
        for i in &field_table_len.to_be_bytes() {
            buffer[index] = *i;
            index += 1;
        }
    }
}

pub type BytesArray = LongStr;

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

impl WriteToBuf for FieldValue {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
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
            FieldValueInner::Decimal(v) => v.write_to_buf(buffer),
            FieldValueInner::LongStr(v) => v.write_to_buf(buffer),
            FieldValueInner::FieldArray(v) => {
                v.write_to_buf(buffer);
            }
            FieldValueInner::FieldTable(v) => {
                v.write_to_buf(buffer);
            }
            FieldValueInner::BytesArray(v) => {
                v.write_to_buf(buffer);
            }
            FieldValueInner::Void => {}
        }
    }
}

pub type FieldTable = HashMap<FieldName, FieldValue>;

impl WriteToBuf for FieldTable {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        let mut index = buffer.len();
        buffer.put_u32(0);
        for (k, v) in self {
            k.write_to_buf(buffer);
            v.write_to_buf(buffer);
        }
        let field_table_len = (buffer.len() - index - std::mem::size_of::<u32>()) as u32;
        // set the true length of the field table
        for i in &field_table_len.to_be_bytes() {
            buffer[index] = *i;
            index += 1;
        }
    }
}


#[derive(Clone)]
pub enum Class {
    Connection,
    Channel,
    Access,
    Exchange,
    Queue,
    Basic,
    Tx,
    Confirm,
    Unknown
}

impl Class {
    pub fn class_id(&self) -> u16 {
        match self {
            Class::Connection => 10,
            Class::Channel => 20,
            Class::Access => 30,
            Class::Exchange => 40,
            Class::Queue => 50,
            Class::Basic => 60,
            Class::Confirm => 85,
            Class::Tx => 90,
            Class::Unknown => 0xffff
        }
    }
}

impl From<u16> for Class {
    fn from(class_id: u16) -> Self {
        match class_id {
            10 => Class::Connection,
            20 => Class::Channel,
            30 => Class::Access,
            40 => Class::Exchange,
            50 => Class::Queue,
            60 => Class::Basic,
            85 => Class::Confirm,
            90 => Class::Tx,
            _  => Class::Unknown
        }
    }
}

impl Default for Class {
    fn default() -> Self {
        Class::Connection
    }
}


pub enum Method {
    ConnectionMethod(ConnectionMethod),
    ChannelMethod(ChannelMethod),
    AccessMethod(AccessMethod),
    ExchangeMethod(ExchangeMethod),
    QueueMethod(QueueMethod),
    BasicMethod(BasicMethod),
    ConfirmMethod(ConfirmMethod),
    TxMethod(TxMethod)
}

impl MethodId for Method {
    fn method_id(&self) -> u16 {
        match self {
            Method::ConnectionMethod(method) => method.method_id(),
            Method::ChannelMethod(method) => method.method_id(),
            Method::AccessMethod(method) => method.method_id(),
            Method::ExchangeMethod(method) => method.method_id(),
            Method::QueueMethod(method) => method.method_id(),
            Method::BasicMethod(method) => method.method_id(),
            Method::ConfirmMethod(method) => method.method_id(),
            Method::TxMethod(method) => method.method_id()
        }
    }
}

impl Default for Method {
    fn default() -> Self {
        Method::ConnectionMethod(ConnectionMethod::default())
    }
}