use crate::method::base::MethodId;

#[derive(Clone, Copy)]
pub enum ConfirmMethod {
    Select,
    SelectOk,
    Unknown
}

impl MethodId for ConfirmMethod {
    fn method_id(&self) -> u16 {
        match self {
            ConfirmMethod::Select => 10,
            ConfirmMethod::SelectOk => 11,
            ConfirmMethod::Unknown => 0xffff
        }
    }
}

impl Default for ConfirmMethod {
    fn default() -> Self {
        ConfirmMethod::Unknown
    }
}

impl From<u16> for ConfirmMethod {
    fn from(method_id: u16) -> Self {
        match method_id {
            10 => ConfirmMethod::Select,
            11 => ConfirmMethod::SelectOk,
            _ => ConfirmMethod::Unknown
        }
    }
}