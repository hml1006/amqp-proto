use crate::method::base::MethodId;

#[derive(Clone, Copy)]
pub enum ConnectionMethod {
    Start,
    StartOk,
    Secure,
    SecureOk,
    Tune,
    TuneOk,
    Open,
    OpenOk,
    Close,
    CloseOk,
    Unknown,
}

impl MethodId for ConnectionMethod {
    fn method_id(&self) -> u16 {
        match self {
            ConnectionMethod::Start => 10,
            ConnectionMethod::StartOk => 11,
            ConnectionMethod::Secure => 20,
            ConnectionMethod::SecureOk => 21,
            ConnectionMethod::Tune => 30,
            ConnectionMethod::TuneOk => 31,
            ConnectionMethod::Open => 40,
            ConnectionMethod::OpenOk => 41,
            ConnectionMethod::Close => 50,
            ConnectionMethod::CloseOk => 51,
            ConnectionMethod::Unknown => 0xffff
        }
    }
}

impl Default for ConnectionMethod {
    fn default() -> Self {
        ConnectionMethod::Unknown
    }
}

impl From<u16> for ConnectionMethod {
    fn from(method_id: u16) -> Self {
        match method_id {
            10 => ConnectionMethod::Start,
            11 => ConnectionMethod::StartOk,
            20 => ConnectionMethod::Secure,
            21 => ConnectionMethod::SecureOk,
            30 => ConnectionMethod::Tune,
            31 => ConnectionMethod::TuneOk,
            40 => ConnectionMethod::Open,
            41 => ConnectionMethod::OpenOk,
            50 => ConnectionMethod::Close,
            51 => ConnectionMethod::CloseOk,
            _  => ConnectionMethod::Unknown
        }
    }
}