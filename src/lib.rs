extern crate config;
extern crate dirs;
extern crate serde;
#[macro_use]
extern crate serde_derive;

pub mod error;
pub mod hooks;
mod settings;
mod wrapper;

use std::env;

pub use error::Result;
pub use hooks::Hook;

pub fn run_hook(hook: Hook) -> Result<()> {
    let home_path = dirs::home_dir().unwrap();
    let git_root_path = env::current_dir().unwrap();

    let config = settings::Settings::init(home_path, git_root_path)?;

    let handler_list = hooks::build(&config);

    println!("hook: {:#?}", &hook);
    println!("config: {:#?}", &config);

    for handler in handler_list {
        if let Err(err) = handler.run(&hook) {
            return Err(err);
        }
    }

    Ok(())
}
