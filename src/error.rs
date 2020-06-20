use std::fmt::Formatter;

pub enum ErrorKind {
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

impl ToString for ErrorKind {
    fn to_string(&self) -> String {
        match self {
            ErrorKind::ReplySuccess => format!("{}, Success", self.code()),
            ErrorKind::ContentTooLarge => format!("{}, Content too large", self.code()),
            ErrorKind::NoConsumers => format!("{}, No consumers", self.code()),
            ErrorKind::ConnectionForced => format!("{}, Connection forced", self.code()),
            ErrorKind::InvalidPath => format!("{}, Invalid path", self.code()),
            ErrorKind::AccessRefused => format!("{}, Access refused", self.code()),
            ErrorKind::NotFound => format!("{}, Not found", self.code()),
            ErrorKind::ResourceLocked => format!("{}, Resource locked", self.code()),
            ErrorKind::PreconditionFailed => format!("{}, Precondition failed", self.code()),
            ErrorKind::FrameError => format!("{}, Frame error", self.code()),
            ErrorKind::SyntaxError => format!("{}, Syntax error", self.code()),
            ErrorKind::CommandInvalid => format!("{}, Command invalid", self.code()),
            ErrorKind::ChannelError => format!("{}, Channel error", self.code()),
            ErrorKind::UnexpectedFrame => format!("{}, Unexpected frame", self.code()),
            ErrorKind::ResourceError => format!("{}, Resource error", self.code()),
            ErrorKind::NotAllowed => format!("{}, Not allowed", self.code()),
            ErrorKind::NotImplemented => format!("{}, Not implemented", self.code()),
            ErrorKind::InternalError => format!("{}, Internal error", self.code()),
        }
    }
}

impl ErrorKind {
    pub fn code(&self) -> u16 {
        match self {
            ErrorKind::ReplySuccess => 200,
            ErrorKind::ContentTooLarge => 311,
            ErrorKind::NoConsumers => 313,
            ErrorKind::ConnectionForced => 320,
            ErrorKind::InvalidPath => 402,
            ErrorKind::AccessRefused => 403,
            ErrorKind::NotFound => 404,
            ErrorKind::ResourceLocked => 405,
            ErrorKind::PreconditionFailed => 406,
            ErrorKind::FrameError => 501,
            ErrorKind::SyntaxError => 502,
            ErrorKind::CommandInvalid => 503,
            ErrorKind::ChannelError => 504,
            ErrorKind::UnexpectedFrame => 505,
            ErrorKind::ResourceError => 506,
            ErrorKind::NotAllowed => 530,
            ErrorKind::NotImplemented => 540,
            ErrorKind::InternalError => 541
        }
    }
}

pub struct Error {
    kind: ErrorKind
}

impl Error {
    pub(crate) fn as_str(&self) -> String {
        self.kind.to_string()
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Error { kind }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl std::error::Error for Error {
}