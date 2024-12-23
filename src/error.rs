/// An error that can occur in this CLI
#[derive(Clone, Debug)]
pub struct Error {
    kind: ErrorKind,
}

#[derive(Clone, Debug)]
pub enum ErrorKind {
    /// Error caused by an invalid CLI mode
    Mode(String),

    /// Error caused by invalid JSON schema for overrides
    Overrides(String),

    /// Error caused by comma-separate, multi-file list being malformed
    CommaSeparated(String),

    /// Error caused by a missing/invalid directory
    Directory(String),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.kind {
            ErrorKind::Mode(ref e) => write!(f, "{}", e),
            ErrorKind::Overrides(ref e) => write!(f, "{}", e),
            ErrorKind::CommaSeparated(ref e) => write!(f, "{}", e),
            ErrorKind::Directory(ref e) => write!(f, "{}", e),
        }
    }
}
