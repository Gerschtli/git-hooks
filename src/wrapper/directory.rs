use error::*;
use std::env;
use std::path::PathBuf;

pub(crate) fn current() -> Result<PathBuf> {
    env::current_dir().wrap_error("failed to get git root directory")
}
