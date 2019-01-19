use std::io;
use wrapper::config;

use std::result::Result as StdResult;
pub(crate) type Result<T> = StdResult<T, Error>;

#[derive(Debug)]
pub(crate) enum Error {
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

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest2::prelude::*;

    mod wrap_error_ext {
        use super::*;

        mod result_with_config_error {
            use super::*;

            #[test]
            fn when_ok() {
                let result: StdResult<u32, config::ConfigError> = Ok(42);
                let wrapped = result.wrap_error("message");

                // TODO: comment in after https://github.com/Valloric/hamcrest2-rust/pull/6
                // assert_that!(wrapped, is(ok()));
                assert_that!(wrapped.unwrap(), is(equal_to(42)));
            }

            #[test]
            fn when_err() {
                let result: StdResult<u32, _> = Err(config::ConfigError::NotFound("x".to_owned()));
                let wrapped = result.wrap_error("message");

                // TODO: comment in after https://github.com/Valloric/hamcrest2-rust/pull/6
                // assert_that!(wrapped, is(err()));
                let error = if let Err(ref err) = wrapped {
                    err
                } else {
                    #[cfg_attr(tarpaulin, skip)]
                    unreachable!();
                };

                match error {
                    Error::InvalidConfig(message, err) => {
                        assert_that!(message, is(equal_to("message")));
                        assert_that!(
                            format!("{:?}", err),
                            is(equal_to("configuration property \"x\" not found"))
                        );
                    },
                    #[cfg_attr(tarpaulin, skip)]
                    _ => assert!(false, "wrong error type"),
                }
            }
        }

        mod result_with_io_error {
            use super::*;

            #[test]
            fn when_ok() {
                let result: StdResult<u32, io::Error> = Ok(42);
                let wrapped = result.wrap_error("message");

                // TODO: comment in after https://github.com/Valloric/hamcrest2-rust/pull/6
                // assert_that!(wrapped, is(ok()));
                assert_that!(wrapped.unwrap(), is(equal_to(42)));
            }

            #[test]
            fn when_err() {
                let result: StdResult<u32, _> = Err(io::Error::new(io::ErrorKind::NotFound, "x"));
                let wrapped = result.wrap_error("message");

                // TODO: comment in after https://github.com/Valloric/hamcrest2-rust/pull/6
                // assert_that!(wrapped, is(err()));
                let error = if let Err(ref err) = wrapped {
                    err
                } else {
                    #[cfg_attr(tarpaulin, skip)]
                    unreachable!();
                };

                match error {
                    Error::IoConfig(message, err) => {
                        assert_that!(message, is(equal_to("message")));
                        assert_that!(
                            format!("{:?}", err),
                            is(equal_to(
                                "Custom { kind: NotFound, error: StringError(\"x\") }"
                            ))
                        );
                    },
                    #[cfg_attr(tarpaulin, skip)]
                    _ => assert!(false, "wrong error type"),
                }
            }
        }

        mod option {
            use super::*;

            #[test]
            fn when_ok() {
                let option = Some(42);
                let wrapped = option.wrap_error("message");

                // TODO: comment in after https://github.com/Valloric/hamcrest2-rust/pull/6
                // assert_that!(wrapped, is(ok()));
                assert_that!(wrapped.unwrap(), is(equal_to(42)));
            }

            #[test]
            fn when_err() {
                let option: Option<u32> = None;
                let wrapped = option.wrap_error("message");

                // TODO: comment in after https://github.com/Valloric/hamcrest2-rust/pull/6
                // assert_that!(wrapped, is(err()));
                let error = if let Err(ref err) = wrapped {
                    err
                } else {
                    #[cfg_attr(tarpaulin, skip)]
                    unreachable!();
                };

                match error {
                    Error::General(message) => {
                        assert_that!(message, is(equal_to("message")));
                    },
                    #[cfg_attr(tarpaulin, skip)]
                    _ => assert!(false, "wrong error type"),
                }
            }
        }
    }
}
