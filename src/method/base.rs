use crate::class::Class;
use crate::error::FrameDecodeErr;
use crate::method::tx::TxMethod;
use crate::method::connection::ConnectionMethod;
use crate::method::channel::ChannelMethod;
use crate::method::access::AccessMethod;
use crate::method::exchange::ExchangeMethod;
use crate::method::queue::QueueMethod;
use crate::method::basic::BasicMethod;
use crate::method::confirm::ConfirmMethod;

pub trait MethodId {
    fn method_id(&self) -> u16;
}

#[derive(Clone, Copy)]
pub enum Method {
    Connection(ConnectionMethod),
    Channel(ChannelMethod),
    Access(AccessMethod),
    Exchange(ExchangeMethod),
    Queue(QueueMethod),
    Basic(BasicMethod),
    Confirm(ConfirmMethod),
    Tx(TxMethod)
}

impl MethodId for Method {
    fn method_id(&self) -> u16 {
        match self {
            Method::Connection(method) => method.method_id(),
            Method::Channel(method) => method.method_id(),
            Method::Access(method) => method.method_id(),
            Method::Exchange(method) => method.method_id(),
            Method::Queue(method) => method.method_id(),
            Method::Basic(method) => method.method_id(),
            Method::Confirm(method) => method.method_id(),
            Method::Tx(method) => method.method_id()
        }
    }
}

impl Default for Method {
    fn default() -> Self {
        Method::Connection(ConnectionMethod::default())
    }
}

pub(crate) fn get_method_type(class: Class, method_id: u16) -> Result<Method, FrameDecodeErr> {
    match class {
        Class::Connection => {
            let method = ConnectionMethod::from(method_id);
            if let ConnectionMethod::Unknown = method {
                return Err(FrameDecodeErr::SyntaxError("unknown method for connection"));
            } else {
                return Ok(Method::Connection(method));
            }
        }
        Class::Channel => {
            let method = ChannelMethod::from(method_id);
            if let ChannelMethod::Unknown = method {
                return Err(FrameDecodeErr::SyntaxError("unknown method for channel"));
            } else {
                return Ok(Method::Channel(method));
            }
        }
        Class::Access => {
            let method = AccessMethod::from(method_id);
            if let AccessMethod::Unknown = method {
                return Err(FrameDecodeErr::SyntaxError("unknown method for access"));
            } else {
                return Ok(Method::Access(method));
            }
        }
        Class::Exchange => {
            let method = ExchangeMethod::from(method_id);
            if let ExchangeMethod::Unknown = method {
                return Err(FrameDecodeErr::SyntaxError("unknown method for exchange"));
            } else {
                return Ok(Method::Exchange(method));
            }
        }
        Class::Queue => {
            let method = QueueMethod::from(method_id);
            if let QueueMethod::Unknown = method {
                return Err(FrameDecodeErr::SyntaxError("unknown method for queue"));
            } else {
                return Ok(Method::Queue(method));
            }
        }
        Class::Basic => {
            let method = BasicMethod::from(method_id);
            if let BasicMethod::Unknown = method {
                return Err(FrameDecodeErr::SyntaxError("unknown method for basic"));
            } else {
                return Ok(Method::Basic(method));
            }
        }
        Class::Tx => {
            let method = TxMethod::from(method_id);
            if let TxMethod::Unknown = method {
                return Err(FrameDecodeErr::SyntaxError("unknown method for tx"));
            } else {
                return Ok(Method::Tx(method));
            }
        }
        Class::Confirm => {
            let method = ConfirmMethod::from(method_id);
            if let ConfirmMethod::Unknown = method {
                return Err(FrameDecodeErr::SyntaxError("unknown method for confirm"));
            } else {
                return Ok(Method::Confirm(method));
            }
        }
        Class::Unknown => return Err(FrameDecodeErr::SyntaxError("unknown class"))
    }
}