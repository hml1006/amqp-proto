use crate::method::base::MethodId;

#[derive(Clone, Copy)]
pub enum ChannelMethod {
    Open,
    OpenOk,
    Flow,
    FlowOk,
    Close,
    CloseOk,
    Unknown
}

impl MethodId for ChannelMethod {
    fn method_id(&self) -> u16 {
        match self {
            ChannelMethod::Open => 10,
            ChannelMethod::OpenOk => 11,
            ChannelMethod::Flow => 20,
            ChannelMethod::FlowOk => 21,
            ChannelMethod::Close => 40,
            ChannelMethod::CloseOk => 41,
            ChannelMethod::Unknown => 0xffff
        }
    }
}

impl Default for ChannelMethod {
    fn default() -> ChannelMethod {
        ChannelMethod::Unknown
    }
}

impl From<u16> for ChannelMethod {
    fn from(method_id: u16) -> Self {
        match method_id {
            10 => ChannelMethod::Open,
            11 => ChannelMethod::OpenOk,
            20 => ChannelMethod::Flow,
            21 => ChannelMethod::FlowOk,
            40 => ChannelMethod::Close,
            41 => ChannelMethod::CloseOk,
            _  => ChannelMethod::Unknown
        }
    }
}