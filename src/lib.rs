#![deny(
    anonymous_parameters,
    bare_trait_objects,
    missing_copy_implementations,
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unsafe_code,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    variant_size_differences
)]
#![cfg_attr(
    feature = "rust-1-31",
    deny(
        clippy::all,
        clippy::complexity,
        clippy::correctness,
        clippy::nursery,
        clippy::pedantic,
        clippy::perf,
        clippy::style,
        elided_lifetimes_in_paths,
        single_use_lifetimes
    )
)]
#![cfg_attr(
    feature = "rust-1-31",
    allow(clippy::filter_map, clippy::non_ascii_literal, deprecated)
)]
#![cfg_attr(all(test, feature = "mocking"), allow(trivial_casts, unsafe_code))]
#![cfg_attr(
    all(test, feature = "mocking"),
    feature(custom_attribute, proc_macro_hygiene)
)]

extern crate config;
extern crate dirs;
#[cfg(test)]
#[macro_use]
extern crate hamcrest2;
extern crate serde;
#[macro_use]
extern crate serde_derive;

mod error;
pub mod hooks;
mod settings;
mod wrapper;

use error::*;
use wrapper::directory;
use wrapper::process;

pub use hooks::Hook;

fn run_handler(hook: Hook) -> Result<bool> {
    let home_path = directory::home()?;
    let git_root_path = directory::current()?;

    let config = settings::Settings::init(home_path, git_root_path)?;
    let handler_list = hooks::build(&config);

    let mut success = true;
    for handler in handler_list {
        if !handler.run(&hook)? {
            success = false;
        }
    }

    Ok(success)
}

pub fn run_hook(hook: Hook) {
    process::exit(match run_handler(hook) {
        Ok(true) => {
            println!("Hook executed successfully!");
            0
        },
        Ok(false) => {
            println!("Checks in hook failed!");
            2
        },
        Err(err) => {
            println!("An error occured!");
            println!("{:#?}", err);
            1
        },
    });
}
