use error::*;
use std::process;

pub(crate) struct Command {
    command: process::Command,
}

impl Command {
    pub(crate) fn new(program: &str, args: &[&str]) -> Self {
        let mut command = process::Command::new(program);
        for arg in args {
            command.arg(arg);
        }

        Self { command }
    }

    pub(crate) fn run(mut self) -> Result<process::ExitStatus> {
        self.command.status().wrap_error("failed to start process")
    }
}
