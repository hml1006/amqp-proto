use property::Property;
use crate::{ShortStr, FieldTable, Timestamp};
use crate::frame::base::{Encode, Property, Decode};
use bytes::{BytesMut, BufMut};
use crate::error::FrameDecodeErr;

#[derive(Property, Default)]
#[property(get(public), set(disable))]
pub struct BasicProperties {
    flags: u32,
    content_type: ShortStr,
    content_encoding: ShortStr,
    headers: FieldTable,
    delivery_mode: u8,
    priority: u8,
    correlation_id: ShortStr,
    reply_to: ShortStr,
    expiration: ShortStr,
    message_id: ShortStr,
    timestamp: Timestamp,
    basic_type: ShortStr,
    user_id: ShortStr,
    app_id: ShortStr,
    cluster_id: ShortStr
}

impl BasicProperties {
    #[inline]
    pub fn set_content_type(&mut self, content_type: ShortStr) {
        self.flags |= BasicProperties::CONTENT_TYPE_FLAG;
        self.content_type = content_type;
    }

    #[inline]
    pub fn set_content_encoding(&mut self, content_encoding: ShortStr) {
        self.flags |= BasicProperties::CONTENT_ENCODING_FLAG;
        self.content_encoding = content_encoding;
    }

    #[inline]
    pub fn set_headers(&mut self, headers: FieldTable) {
        self.flags |= BasicProperties::HEADERS_FLAG;
        self.headers = headers;
    }

    #[inline]
    pub fn set_delivery_mode(&mut self, delivery_mode: u8) {
        self.flags |= BasicProperties::DELIVERY_FLAG;
        self.delivery_mode = delivery_mode;
    }

    #[inline]
    pub fn set_priority(&mut self, priority: u8) {
        self.flags |= BasicProperties::PRIORITY_FLAG;
        self.priority = priority;
    }

    #[inline]
    pub fn set_correlation_id(&mut self, correlation_id: ShortStr) {
        self.flags |= BasicProperties::CORRELATION_ID_FLAG;
        self.correlation_id = correlation_id;
    }

    #[inline]
    pub fn set_reply_to(&mut self, reply_to: ShortStr) {
        self.flags |= BasicProperties::REPLY_TO_FLAG;
        self.reply_to = reply_to;
    }

    #[inline]
    pub fn set_expiration(&mut self, expiration: ShortStr) {
        self.flags |= BasicProperties::EXPIRATION_FLAG;
        self.expiration = expiration;
    }

    #[inline]
    pub fn set_message_id(&mut self, message_id: ShortStr) {
        self.flags |= BasicProperties::MESSAGE_ID_FLAG;
        self.message_id = message_id;
    }

    #[inline]
    pub fn set_timestamp(&mut self, timestamp: Timestamp) {
        self.flags |= BasicProperties::TIMESTAMP_FLAG;
        self.timestamp = timestamp;
    }

    #[inline]
    pub fn set_basic_type(&mut self, basic_type: ShortStr) {
        self.flags |= BasicProperties::BASIC_TYPE_FLAG;
        self.basic_type = basic_type;
    }

    #[inline]
    pub fn set_user_id(&mut self, user_id: ShortStr) {
        self.flags |= BasicProperties::USER_ID_FLAG;
        self.user_id = user_id;
    }

    #[inline]
    pub fn set_app_id(&mut self, app_id: ShortStr) {
        self.flags |= BasicProperties::APP_ID_FLAG;
        self.app_id = app_id;
    }

    #[inline]
    pub fn set_cluster_id(&mut self, cluster_id: ShortStr) {
        self.flags |= BasicProperties::CLUSTER_ID_FLAG;
        self.cluster_id = cluster_id;
    }
}

impl Encode for BasicProperties {
    fn encode(&self, buffer: &mut BytesMut) {
        buffer.put_u32(self.flags);
        if self.flags & BasicProperties::CONTENT_TYPE_FLAG != 0 {
            self.content_type.encode(buffer);
        }

        if self.flags & BasicProperties::CONTENT_ENCODING_FLAG != 0 {
            self.content_encoding.encode(buffer);
        }

        if self.flags & BasicProperties::HEADERS_FLAG != 0 {
            self.headers.encode(buffer);
        }

        if self.flags & BasicProperties::DELIVERY_FLAG != 0 {
            buffer.put_u8(self.delivery_mode);
        }

        if self.flags & BasicProperties::PRIORITY_FLAG != 0 {
            buffer.put_u8(self.priority);
        }

        if self.flags & BasicProperties::CORRELATION_ID_FLAG != 0 {
            self.correlation_id.encode(buffer);
        }

        if self.flags & BasicProperties::REPLY_TO_FLAG != 0 {
            self.reply_to.encode(buffer);
        }

        if self.flags & BasicProperties::EXPIRATION_FLAG != 0 {
            self.expiration.encode(buffer);
        }

        if self.flags & BasicProperties::MESSAGE_ID_FLAG != 0 {
            self.message_id.encode(buffer);
        }

        if self.flags & BasicProperties::TIMESTAMP_FLAG != 0 {
            buffer.put_u64(self.timestamp);
        }

        if self.flags & BasicProperties::BASIC_TYPE_FLAG != 0 {
            self.basic_type.encode(buffer);
        }

        if self.flags & BasicProperties::USER_ID_FLAG != 0 {
            self.user_id.encode(buffer);
        }

        if self.flags & BasicProperties::APP_ID_FLAG != 0 {
            self.app_id.encode(buffer);
        }

        if self.flags & BasicProperties::CLUSTER_ID_FLAG != 0 {
            self.cluster_id.encode(buffer);
        }
    }
}

impl Decode<Property> for BasicProperties {
    fn decode(buffer: &[u8]) -> Result<(&[u8], Property), FrameDecodeErr>{
        let (buffer, flags) = match u32::decode(buffer) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicProperties flags -> {}", e))),
        };
        let mut properties = BasicProperties::default();
        let buffer = if flags & BasicProperties::CONTENT_TYPE_FLAG != 0 {
            let (buffer, content_type) = match ShortStr::decode(buffer) {
                Ok(ret) => ret,
                Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicProperties content type -> {}", e))),
            };
            properties.set_content_type(content_type);
            buffer
        } else { buffer };

        let buffer = if flags & BasicProperties::CONTENT_ENCODING_FLAG != 0 {
            let (buffer, content_encoding) = match ShortStr::decode(buffer) {
                Ok(ret) => ret,
                Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicProperties content-encoding -> {}", e)))
            };
            properties.set_content_encoding(content_encoding);
            buffer
        } else { buffer };

        let buffer = if flags & BasicProperties::HEADERS_FLAG != 0 {
            let (buffer, headers) = match FieldTable::decode(buffer) {
                Ok(ret) => ret,
                Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicProperties headers -> {}", e)))
            };
            properties.set_headers(headers);
            buffer
        } else { buffer };

        let buffer = if flags & BasicProperties::DELIVERY_FLAG != 0 {
            let (buffer, delivery_mode) = match u8::decode(buffer) {
                Ok(ret) => ret,
                Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicProperties delivery mode -> {}", e)))
            };
            properties.set_delivery_mode(delivery_mode);
            buffer
        } else { buffer };

        let buffer = if flags & BasicProperties::PRIORITY_FLAG != 0 {
            let (buffer, priority) = match u8::decode(buffer) {
                Ok(ret) => ret,
                Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicPropertiespriority -> {}", e)))
            };
            properties.set_priority(priority);
            buffer
        } else { buffer };

        let buffer = if flags & BasicProperties::CORRELATION_ID_FLAG != 0 {
            let (buffer, correlation_id) = match ShortStr::decode(buffer) {
                Ok(ret) => ret,
                Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicProperties correlation id -> {}", e)))
            };
            properties.set_correlation_id(correlation_id);
            buffer
        } else { buffer };

        let buffer = if flags & BasicProperties::REPLY_TO_FLAG != 0 {
            let (buffer, reply_to) = match ShortStr::decode(buffer) {
                Ok(ret) => ret,
                Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicProperties reply_to -> {}", e)))
            };
            properties.set_reply_to(reply_to);
            buffer
        } else { buffer };

        let buffer = if flags & BasicProperties::EXPIRATION_FLAG != 0 {
            let (buffer, expiration) = match ShortStr::decode(buffer) {
                Ok(ret) => ret,
                Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicProperties expiration -> {}", e)))
            };
            properties.set_expiration(expiration);
            buffer
        } else { buffer };

        let buffer = if flags & BasicProperties::MESSAGE_ID_FLAG != 0 {
            let (buffer, message_id) = match ShortStr::decode(buffer) {
                Ok(ret) => ret,
                Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicProperties message_id -> {}", e)))
            };
            properties.set_message_id(message_id);
            buffer
        } else { buffer };

        let buffer = if flags & BasicProperties::TIMESTAMP_FLAG != 0 {
            let (buffer, timestamp) = match u64::decode(buffer) {
                Ok(ret) => ret,
                Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicProperties timestamp -> {}", e)))
            };
            properties.set_timestamp(timestamp);
            buffer
        } else { buffer };

        let buffer = if flags & BasicProperties::BASIC_TYPE_FLAG != 0 {
            let (buffer, basic_type) = match ShortStr::decode(buffer) {
                Ok(ret) => ret,
                Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicProperties basic_type -> {}", e)))
            };
            properties.set_basic_type(basic_type);
            buffer
        } else { buffer };

        let buffer = if flags & BasicProperties::USER_ID_FLAG != 0 {
            let (buffer, user_id) = match ShortStr::decode(buffer) {
                Ok(ret) => ret,
                Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicProperties user_id -> {}", e)))
            };
            properties.set_user_id(user_id);
            buffer
        } else { buffer };

        let buffer = if flags & BasicProperties::APP_ID_FLAG != 0 {
            let (buffer, app_id) = match ShortStr::decode(buffer) {
                Ok(ret) => ret,
                Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicProperties app_id -> {}", e)))
            };
            properties.set_app_id(app_id);
            buffer
        } else { buffer };

        let buffer = if flags & BasicProperties::CLUSTER_ID_FLAG != 0 {
            let (buffer, cluster_id) = match ShortStr::decode(buffer) {
                Ok(ret) => ret,
                Err(e) => return Err(FrameDecodeErr::DecodeError(format!("decode BasicProperties cluster_id -> {}", e)))
            };
            properties.set_cluster_id(cluster_id);
            buffer
        } else { buffer };
        Ok((buffer, Property::Basic(properties)))
    }
}

impl BasicProperties {
    const CONTENT_TYPE_FLAG: u32 = 1 << 15;
    const CONTENT_ENCODING_FLAG: u32 = 1 << 14;
    const HEADERS_FLAG: u32 = 1 << 13;
    const DELIVERY_FLAG: u32 = 1 << 12;
    const PRIORITY_FLAG: u32 = 1 << 11;
    const CORRELATION_ID_FLAG: u32 = 1 << 10;
    const REPLY_TO_FLAG: u32 = 1 << 9;
    const EXPIRATION_FLAG: u32 = 1 << 8;
    const MESSAGE_ID_FLAG: u32 = 1 << 7;
    const TIMESTAMP_FLAG: u32 = 1 << 6;
    const BASIC_TYPE_FLAG: u32 = 1 << 5;
    const USER_ID_FLAG: u32 = 1 << 4;
    const APP_ID_FLAG: u32 = 1 << 3;
    const CLUSTER_ID_FLAG: u32 = 1 << 2;
}
