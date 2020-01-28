use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;

pub type Result<T> = std::result::Result<T, ImtError>;

#[derive(Debug)]
pub enum ImtError {}

impl Error for ImtError {}

impl Display for ImtError
where
    ImtError: Error,
{
    fn fmt(&self, f: &mut Formatter) -> std::result::Result<(), std::fmt::Error> {
        f.write_str(&self.to_string())
    }
}
