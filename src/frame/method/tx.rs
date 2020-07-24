use bytes::BytesMut;
use crate::frame::base::{Encode, Arguments, Decode};
use crate::error::FrameDecodeErr;

pub struct TxSelect;

impl Encode for TxSelect {
    #[inline]
    fn encode(&self, _: &mut BytesMut) {
    }
}

impl Decode<Arguments> for TxSelect {
    #[inline]
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        Ok((buffer, Arguments::TxSelect(TxSelect)))
    }
}

pub struct TxSelectOk;

impl Encode for TxSelectOk {
    #[inline]
    fn encode(&self, _: &mut BytesMut) {
    }
}

impl Decode<Arguments> for TxSelectOk {
    #[inline]
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        Ok((buffer, Arguments::TxSelectOk(TxSelectOk)))
    }
}

pub struct TxCommit;

impl Encode for TxCommit {
    #[inline]
    fn encode(&self, _: &mut BytesMut) {
    }
}

impl Decode<Arguments> for TxCommit {
    #[inline]
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        Ok((buffer, Arguments::TxCommit(TxCommit)))
    }
}

pub struct TxCommitOk;

impl Encode for TxCommitOk {
    #[inline]
    fn encode(&self, _: &mut BytesMut) {
    }
}

impl Decode<Arguments> for TxCommitOk {
    #[inline]
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        Ok((buffer, Arguments::TxCommitOk(TxCommitOk)))
    }
}

pub struct TxRollback;

impl Encode for TxRollback {
    #[inline]
    fn encode(&self, _: &mut BytesMut) {
    }
}

impl Decode<Arguments> for TxRollback {
    #[inline]
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        Ok((buffer, Arguments::TxRollback(TxRollback)))
    }
}

pub struct TxRollbackOk;

impl Encode for TxRollbackOk {
    #[inline]
    fn encode(&self, _: &mut BytesMut) {
    }
}

impl Decode<Arguments> for TxRollbackOk {
    fn decode(buffer: &[u8]) -> Result<(&[u8], Arguments), FrameDecodeErr>{
        Ok((buffer, Arguments::TxRollbackOk(TxRollbackOk)))
    }
}
