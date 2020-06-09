use std::fmt::Formatter;

pub enum ErrorKind {
    BadFrame,
    StrTooLong,
}

impl ErrorKind {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            BadFrame => "Bad Frame",
            StrTooLong => "String Too Long",
        }
    }
}

pub struct Error {
    kind: ErrorKind
}

impl Error {
    pub(crate) fn as_str(&self) -> &'static str {
        self.kind.as_str()
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Error { kind }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::error::Error for Error {

}