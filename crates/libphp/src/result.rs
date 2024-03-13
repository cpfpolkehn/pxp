use std::ffi::NulError;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Clone)]
pub enum Error {
    InvalidCString,
}

impl From<NulError> for Error {
    fn from(_: NulError) -> Self {
        Error::InvalidCString
    }
}