use bytes::{BytesMut, BufMut};
use crate::error;
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::{Hasher, Hash};

const DEFAULT_BYTES_CAPACITY: usize = 8;
// amqp0-9-1 field name length allowed is 128
const MAX_FIELD_NAME_LEN: usize = 128;
// max long string bytes length allowed
const MAX_LONG_STR_LEN: usize = 64 * 1024;

pub type Timestamp = u64;

pub trait WriteToBuf {
    // write data to bytes buffer
    fn write_to_buf(&self, buffer: &mut BytesMut);
}

#[derive(Debug, PartialEq, Eq)]
pub struct ShortStr {
    len: u8,
    value: String
}

impl std::hash::Hash for ShortStr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl ShortStr {
    // build a ShortStr from bytes
    #[inline]
    pub fn with_bytes(bytes: &[u8]) -> Result<Self, error::Error>{
        if bytes.len() > std::u8::MAX as usize {
            return Err(error::Error::from(error::ErrorKind::StrTooLong));
        }

        Ok(ShortStr { len: bytes.len() as u8, value: String::from_utf8_lossy(bytes).to_string() })

    }
}

impl WriteToBuf for ShortStr {
    #[inline]
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u8(self.len);
        buffer.extend_from_slice(&self.value.as_bytes());
    }
}

#[derive(Debug)]
pub struct LongStr {
    len: u32,
    value: String
}

impl LongStr {
    // build a LongStr from bytes, the length will be convert to big endian
    #[inline]
    pub fn with_bytes(bytes: &[u8]) -> Result<LongStr, crate::error::Error> {
        if bytes.len() > MAX_LONG_STR_LEN {
            Err(crate::error::Error::from(error::ErrorKind::StrTooLong))
        } else {
            Ok(LongStr {len: bytes.len() as u32, value: String::from_utf8_lossy(bytes).to_string()})
        }
    }
}

impl WriteToBuf for LongStr {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u32(self.len);
        buffer.extend_from_slice(self.value.as_bytes());
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
            FieldValueKind::Void => b'V'
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
    pub fn with_bytes(bytes: &[u8]) -> Result<FieldName, error::Error> {
        // field name first letter should be '$'  '#' or letter
        let is_start_char_ok = match bytes[0] {
            b'$' | b'#' => true,
            b'a'..=b'z' | b'A'..=b'Z' => true,
            _ => false
        };

        if !is_start_char_ok {
            return Err(error::Error::from(error::ErrorKind::WrongShortStrFirstLetter));
        }

        // max field name length is 128
        if bytes.len() > MAX_FIELD_NAME_LEN {
            return Err(error::Error::from(error::ErrorKind::StrTooLong));
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
        // placeholder for array bytes length
        buffer.put_u32(0);
        // save old length
        let old_len = buffer.len() as u32;
        for item in self {
            item.write_to_buf(buffer);
        }
        let arr_bytes_len = buffer.len() as u32 - old_len;
        // set length from beginning
        unsafe {
            let ptr = buffer.as_mut_ptr() as isize + old_len as isize - std::mem::size_of::<u32>() as isize;
            let src = &arr_bytes_len.to_be_bytes();
            std::ptr::copy(src.as_ptr(), ptr as *mut u8, std::mem::size_of::<u32>());
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
        // placeholder length
        buffer.put_u32(0);
        // save length
        let old_len = buffer.len();
        for (k, v) in self {
            k.write_to_buf(buffer);
            v.write_to_buf(buffer);
        }
        let arr_bytes_len = (buffer.len() - old_len) as u32;
        // set length from beginning
        unsafe {
            let ptr = buffer.as_mut_ptr() as isize + old_len as isize - std::mem::size_of::<u32>() as isize;
            let src = &arr_bytes_len.to_be_bytes();
            std::ptr::copy(src.as_ptr(), ptr as *mut u8, std::mem::size_of::<u32>());
        }
    }
}

