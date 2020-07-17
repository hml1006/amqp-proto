use property::Property;
use bytes::{BytesMut, BufMut};
use crate::common::{WriteToBuf, MethodId};

pub struct TxSelect;

impl WriteToBuf for TxSelect {
    #[inline]
    fn write_to_buf(&self, _: &mut BytesMut) {
    }
}

pub struct TxSelectOk;

impl WriteToBuf for TxSelectOk {
    #[inline]
    fn write_to_buf(&self, _: &mut BytesMut) {
    }
}

pub struct TxCommit;

impl WriteToBuf for TxCommit {
    #[inline]
    fn write_to_buf(&self, _: &mut BytesMut) {
    }
}

pub struct TxCommitOk;

impl WriteToBuf for TxCommitOk {
    #[inline]
    fn write_to_buf(&self, _: &mut BytesMut) {
    }
}

pub struct TxRollback;

impl WriteToBuf for TxRollback {
    #[inline]
    fn write_to_buf(&self, _: &mut BytesMut) {
    }
}

pub struct TxRollbackOk;

impl WriteToBuf for TxRollbackOk {
    #[inline]
    fn write_to_buf(&self, _: &mut BytesMut) {
    }
}

#[derive(Property, Default)]
#[property(get(public), set(public))]
pub struct TxProperties {
    flags: u32,
}

impl WriteToBuf for TxProperties {
    fn write_to_buf(&self, buffer: &mut BytesMut) {
        buffer.put_u32(self.flags);
    }
}


pub enum TxMethod {
    Select,
    SelectOk,
    Commit,
    CommitOk,
    Rollback,
    RollbackOk,
    Unknown
}

impl MethodId for TxMethod {
    fn method_id(&self) -> u16 {
        match self {
            TxMethod::Select => 10,
            TxMethod::SelectOk => 11,
            TxMethod::Commit => 20,
            TxMethod::CommitOk => 21,
            TxMethod::Rollback => 30,
            TxMethod::RollbackOk => 31,
            TxMethod::Unknown => 0xffff
        }
    }
}

impl Default for TxMethod {
    fn default() -> Self {
        TxMethod::Unknown
    }
}

impl From<u16> for TxMethod {
    fn from(method_id: u16) -> Self {
        match method_id {
            10 => TxMethod::Select,
            11 => TxMethod::SelectOk,
            20 => TxMethod::Commit,
            21 => TxMethod::CommitOk,
            30 => TxMethod::Rollback,
            31 => TxMethod::RollbackOk,
            _  => TxMethod::Unknown
        }
    }
}