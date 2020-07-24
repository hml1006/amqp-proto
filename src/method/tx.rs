use crate::method::base::MethodId;

#[derive(Clone, Copy)]
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