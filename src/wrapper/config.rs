use config;
use error::*;
use serde::Deserialize;
use std::path::PathBuf;

pub(crate) use config::ConfigError;
pub(crate) use config::Value;

pub(crate) struct Config {
    config: config::Config,
}

impl Config {
    pub(crate) fn new() -> Self {
        Self {
            config: config::Config::new(),
        }
    }

    pub(crate) fn set_default<T>(&mut self, key: &str, value: T) -> Result<()>
    where
        T: Into<Value>,
    {
        self.config
            .set_default(key, value)
            .wrap_error("set default failed")?;

        Ok(())
    }

    pub(crate) fn set_path(&mut self, path_buf: PathBuf) -> Result<()> {
        let path = path_buf.as_path();
        if path.exists() {
            self.config
                .merge(config::File::from(path))
                .wrap_error("merge config file failed")?;
        }

        Ok(())
    }

    #[allow(single_use_lifetimes)] // FIXME
    pub(crate) fn try_into<'de, T: Deserialize<'de>>(self) -> Result<T> {
        self.config
            .try_into()
            .wrap_error("failed to build settings object")
    }
}
