use std::io;
use wrapper::config;

pub use std::result::Result as StdResult;
pub type Result<T> = StdResult<T, Error>;

#[derive(Debug)]
pub enum Error {
    HookFailed,
    General(String),
    InvalidConfig(String, Box<config::ConfigError>),
    IoConfig(String, io::Error),
}

pub(crate) trait WrapErrorExt<T> {
    fn wrap_error<M>(self, message: M) -> Result<T>
    where
        M: Into<String>;
}

impl<T> WrapErrorExt<T> for StdResult<T, config::ConfigError> {
    fn wrap_error<M>(self, message: M) -> Result<T>
    where
        M: Into<String>,
    {
        self.map_err(|error| Error::InvalidConfig(message.into(), error.into()))
    }
}

impl<T> WrapErrorExt<T> for StdResult<T, io::Error> {
    fn wrap_error<M>(self, message: M) -> Result<T>
    where
        M: Into<String>,
    {
        self.map_err(|error| Error::IoConfig(message.into(), error))
    }
}

impl<T> WrapErrorExt<T> for Option<T> {
    fn wrap_error<M>(self, message: M) -> Result<T>
    where
        M: Into<String>,
    {
        self.ok_or_else(|| Error::General(message.into()))
    }
}
