use crate::method::base::MethodId;

#[derive(Clone, Copy)]
pub enum AccessMethod {
    Request,
    RequestOk,
    Unknown
}

impl MethodId for AccessMethod {
    #[inline]
    fn method_id(&self) -> u16 {
        match self {
            AccessMethod::Request => 10,
            AccessMethod::RequestOk => 11,
            AccessMethod::Unknown => 0xffff
        }
    }
}

impl Default for AccessMethod {
    #[inline]
    fn default() -> Self {
        AccessMethod::Unknown
    }
}

impl From<u16> for AccessMethod {
    #[inline]
    fn from(method_id: u16) -> Self {
        match method_id {
            10 => AccessMethod::Request,
            11 => AccessMethod::RequestOk,
            _  => AccessMethod::Unknown
        }
    }
}