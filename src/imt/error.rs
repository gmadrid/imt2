use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;

pub type Result<T> = std::result::Result<T, ImtError>;

#[derive(Debug)]
pub enum ImtError {
    IOError(std::io::Error),
    WalkDirError(walkdir::Error),
}

impl Error for ImtError {}

impl Display for ImtError
where
    ImtError: Error,
{
    fn fmt(&self, f: &mut Formatter) -> std::result::Result<(), std::fmt::Error> {
        f.write_str(&self.to_string())
    }
}

impl From<std::io::Error> for ImtError {
    fn from(err: std::io::Error) -> Self {
        ImtError::IOError(err)
    }
}

impl From<walkdir::Error> for ImtError {
    fn from(err: walkdir::Error) -> Self {
        ImtError::WalkDirError(err)
    }
}
