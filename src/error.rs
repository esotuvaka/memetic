/// An error that can occur in this CLI
#[derive(Debug)]
pub enum Error {
    Struct(String),
    Io(std::io::Error),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Struct(e) => write!(f, "Struct error: {}", e),
            Error::Io(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}
