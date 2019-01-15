pub(super) mod command;

use error::*;
use settings;

pub(super) trait Handler {
    fn pre_push(&self) -> Result<bool> {
        Ok(true)
    }

    fn run(&self, hook: &Hook) -> Result<bool> {
        match hook {
            Hook::PrePush => self.pre_push(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Hook {
    PrePush,
}

macro_rules! handler {
    ( $( $mod:ident ),* ) => {
        pub(super) fn build(config: &settings::Settings) -> Vec<Box<dyn Handler>> {
            let mut handler_list: Vec<Box<dyn Handler>> = vec![];

            $(
                if let Some(handler) = $mod::build(&config.$mod) {
                    handler_list.push(Box::new(handler));
                }
            )*

            handler_list
        }
    }
}

handler!(command);
