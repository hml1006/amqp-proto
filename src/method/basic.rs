use crate::method::base::MethodId;

#[derive(Clone, Copy)]
pub enum BasicMethod {
    Qos,
    QosOk,
    Consume,
    ConsumeOk,
    Cancel,
    CancelOk,
    Publish,
    Return,
    Deliver,
    Get,
    GetOk,
    GetEmpty,
    Ack,
    Reject,
    RecoverAsync,
    Recover,
    RecoverOk,
    Nack,
    Unknown
}

impl MethodId for BasicMethod {
    fn method_id(&self) -> u16 {
        match self {
            BasicMethod::Qos => 10,
            BasicMethod::QosOk => 11,
            BasicMethod::Consume => 20,
            BasicMethod::ConsumeOk => 21,
            BasicMethod::Cancel => 30,
            BasicMethod::CancelOk => 31,
            BasicMethod::Publish => 40,
            BasicMethod::Return => 50,
            BasicMethod::Deliver => 60,
            BasicMethod::Get => 70,
            BasicMethod::GetOk => 71,
            BasicMethod::GetEmpty => 72,
            BasicMethod::Ack => 80,
            BasicMethod::Reject => 90,
            BasicMethod::RecoverAsync => 100,
            BasicMethod::Recover => 110,
            BasicMethod::RecoverOk => 111,
            BasicMethod::Nack => 120,
            BasicMethod::Unknown => 0xffff
        }
    }
}

impl Default for BasicMethod {
    fn default() -> Self {
        BasicMethod::Unknown
    }
}

impl From<u16> for BasicMethod {
    fn from(method_id: u16) -> Self {
        match method_id {
            10 => BasicMethod::Qos,
            11 => BasicMethod::QosOk,
            20 => BasicMethod::Consume,
            21 => BasicMethod::ConsumeOk,
            30 => BasicMethod::Cancel,
            31 => BasicMethod::CancelOk,
            40 => BasicMethod::Publish,
            50 => BasicMethod::Return,
            60 => BasicMethod::Deliver,
            70 => BasicMethod::Get,
            71 => BasicMethod::GetOk,
            72 => BasicMethod::GetEmpty,
            80 => BasicMethod::Ack,
            90 => BasicMethod::Reject,
            100 => BasicMethod::RecoverAsync,
            110 => BasicMethod::Recover,
            111 => BasicMethod::RecoverOk,
            _  => BasicMethod::Unknown
        }
    }
}