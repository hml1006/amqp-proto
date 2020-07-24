use crate::method::base::MethodId;

#[derive(Clone, Copy)]
pub enum ExchangeMethod {
    Declare,
    DeclareOk,
    Delete,
    DeleteOk,
    Bind,
    BindOk,
    Unbind,
    UnbindOk,
    Unknown
}

impl MethodId for ExchangeMethod {
    fn method_id(&self) -> u16 {
        match self {
            ExchangeMethod::Declare => 10,
            ExchangeMethod::DeclareOk => 11,
            ExchangeMethod::Delete => 20,
            ExchangeMethod::DeleteOk => 21,
            ExchangeMethod::Bind => 30,
            ExchangeMethod::BindOk => 31,
            ExchangeMethod::Unbind => 40,
            ExchangeMethod::UnbindOk => 51,
            ExchangeMethod::Unknown => 0xffff
        }
    }
}

impl Default for ExchangeMethod {
    fn default() -> Self {
        ExchangeMethod::Unknown
    }
}

impl From<u16> for ExchangeMethod {
    fn from(method_id: u16) -> Self {
        match method_id {
            10 => ExchangeMethod::Declare,
            11 => ExchangeMethod::DeclareOk,
            20 => ExchangeMethod::Delete,
            21 => ExchangeMethod::DeleteOk,
            30 => ExchangeMethod::Bind,
            31 => ExchangeMethod::BindOk,
            40 => ExchangeMethod::Unbind,
            51 => ExchangeMethod::UnbindOk,
            _  => ExchangeMethod::Unknown
        }
    }
}