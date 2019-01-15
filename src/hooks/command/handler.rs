use super::ConfigEntry;
use error::*;
use hooks;

pub(in hooks) struct Handler {
    config: ConfigEntry,
}

impl Handler {
    pub(super) fn new(config: ConfigEntry) -> Self {
        Self { config }
    }
}

impl hooks::Handler for Handler {
    fn pre_push(&self) -> Result<bool> {
        for x in &self.config.pre_push {
            println!("executing {}", x);
        }

        Ok(true)
    }
}
