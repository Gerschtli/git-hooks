use wrapper::config;

pub use std::result::Result as StdResult;
pub type Result<T> = StdResult<T, Error>;

#[derive(Debug)]
pub enum Error {
    HookFailed,
    InvalidConfig(String, config::ConfigError),
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
        self.map_err(|error| Error::InvalidConfig(message.into(), error))
    }
}
