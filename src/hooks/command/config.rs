use error::*;
use settings::ConfigType;
use wrapper::config;

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct ConfigEntry {
    pub(super) enabled: bool,
    #[serde(rename = "pre-push")]
    pub(super) pre_push: Vec<String>,
}

impl ConfigType for ConfigEntry {
    fn set_default(config: &mut config::Config) -> Result<()> {
        config.set_default("command.enabled", false)?;
        config.set_default("command.pre-push", Vec::<String>::new())?;

        Ok(())
    }
}
