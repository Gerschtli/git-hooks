pub(crate) mod config;
pub(super) mod handler;

pub(crate) use self::config::ConfigEntry;
pub(super) use self::handler::Handler;

pub(super) fn build(config: &ConfigEntry) -> Option<Handler> {
    if config.enabled {
        Some(Handler::new(config.clone()))
    } else {
        None
    }
}
