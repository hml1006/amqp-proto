use std::fmt::Formatter;

pub enum AmqpErrorKind {
    ReplySuccess,
    ContentTooLarge,        // channel
    NoConsumers,            // channel
    ConnectionForced,       // connection
    InvalidPath,            // connection
    AccessRefused,          // channel
    NotFound,               // channel
    ResourceLocked,         // channel
    PreconditionFailed,     // channel
    FrameError,             // connection
    SyntaxError,            // connection
    CommandInvalid,         // connection
    ChannelError,           // connection
    UnexpectedFrame,        // connection
    ResourceError,          // connection
    NotAllowed,             // connection
    NotImplemented,         // connection
    InternalError,          // connection
}

impl ToString for AmqpErrorKind {
    fn to_string(&self) -> String {
        match self {
            AmqpErrorKind::ReplySuccess => format!("{}, Success", self.code()),
            AmqpErrorKind::ContentTooLarge => format!("{}, Content too large", self.code()),
            AmqpErrorKind::NoConsumers => format!("{}, No consumers", self.code()),
            AmqpErrorKind::ConnectionForced => format!("{}, Connection forced", self.code()),
            AmqpErrorKind::InvalidPath => format!("{}, Invalid path", self.code()),
            AmqpErrorKind::AccessRefused => format!("{}, Access refused", self.code()),
            AmqpErrorKind::NotFound => format!("{}, Not found", self.code()),
            AmqpErrorKind::ResourceLocked => format!("{}, Resource locked", self.code()),
            AmqpErrorKind::PreconditionFailed => format!("{}, Precondition failed", self.code()),
            AmqpErrorKind::FrameError => format!("{}, Frame error", self.code()),
            AmqpErrorKind::SyntaxError => format!("{}, Syntax error", self.code()),
            AmqpErrorKind::CommandInvalid => format!("{}, Command invalid", self.code()),
            AmqpErrorKind::ChannelError => format!("{}, Channel error", self.code()),
            AmqpErrorKind::UnexpectedFrame => format!("{}, Unexpected frame", self.code()),
            AmqpErrorKind::ResourceError => format!("{}, Resource error", self.code()),
            AmqpErrorKind::NotAllowed => format!("{}, Not allowed", self.code()),
            AmqpErrorKind::NotImplemented => format!("{}, Not implemented", self.code()),
            AmqpErrorKind::InternalError => format!("{}, Internal error", self.code()),
        }
    }
}

impl AmqpErrorKind {
    pub fn code(&self) -> u16 {
        match self {
            AmqpErrorKind::ReplySuccess => 200,
            AmqpErrorKind::ContentTooLarge => 311,
            AmqpErrorKind::NoConsumers => 313,
            AmqpErrorKind::ConnectionForced => 320,
            AmqpErrorKind::InvalidPath => 402,
            AmqpErrorKind::AccessRefused => 403,
            AmqpErrorKind::NotFound => 404,
            AmqpErrorKind::ResourceLocked => 405,
            AmqpErrorKind::PreconditionFailed => 406,
            AmqpErrorKind::FrameError => 501,
            AmqpErrorKind::SyntaxError => 502,
            AmqpErrorKind::CommandInvalid => 503,
            AmqpErrorKind::ChannelError => 504,
            AmqpErrorKind::UnexpectedFrame => 505,
            AmqpErrorKind::ResourceError => 506,
            AmqpErrorKind::NotAllowed => 530,
            AmqpErrorKind::NotImplemented => 540,
            AmqpErrorKind::InternalError => 541
        }
    }
}

pub struct AmqpError {
    kind: AmqpErrorKind
}

impl AmqpError {
    pub fn kind(&self) -> &AmqpErrorKind {
        &self.kind
    }
}

impl From<AmqpErrorKind> for AmqpError {
    fn from(kind: AmqpErrorKind) -> Self {
        AmqpError { kind }
    }
}

impl std::fmt::Display for AmqpError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl std::fmt::Debug for AmqpError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl std::error::Error for AmqpError {
}