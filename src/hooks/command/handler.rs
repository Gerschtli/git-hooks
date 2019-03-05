use super::ConfigEntry;
use error::*;
use hooks;
use wrapper::process;

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
        let mut success = true;
        for c in &self.config.pre_push {
            println!("run {}", c.command);
            let result = process::Command::new("sh", &["-c", &c.command]).run()?;

            if result {
                println!("success: {}", c.command);
            } else {
                println!("failed: {}", c.command);
                success = false;
            }
        }

        Ok(success)
    }
}
