use crate::method::base::MethodId;

#[derive(Clone, Copy)]
pub enum QueueMethod {
    Declare,
    DeclareOk,
    Bind,
    BindOk,
    Unbind,
    UnbindOk,
    Purge,
    PurgeOk,
    Delete,
    DeleteOk,
    Unknown
}

impl MethodId for QueueMethod {
    fn method_id(&self) -> u16 {
        match self {
            QueueMethod::Declare => 10,
            QueueMethod::DeclareOk => 11,
            QueueMethod::Bind => 20,
            QueueMethod::BindOk => 21,
            QueueMethod::Unbind => 50,
            QueueMethod::UnbindOk => 51,
            QueueMethod::Purge => 30,
            QueueMethod::PurgeOk => 31,
            QueueMethod::Delete => 40,
            QueueMethod::DeleteOk => 41,
            QueueMethod::Unknown => 0xffff
        }
    }
}

impl Default for QueueMethod {
    fn default() -> Self {
        QueueMethod::Unknown
    }
}

impl From<u16> for QueueMethod {
    fn from(method_id: u16) -> Self {
        match method_id {
            10 => QueueMethod::Declare,
            11 => QueueMethod::DeclareOk,
            20 => QueueMethod::Bind,
            21 => QueueMethod::BindOk,
            50 => QueueMethod::Unbind,
            51 => QueueMethod::UnbindOk,
            30 => QueueMethod::Purge,
            31 => QueueMethod::PurgeOk,
            40 => QueueMethod::Delete,
            41 => QueueMethod::DeleteOk,
            _  => QueueMethod::Unknown
        }
    }
}