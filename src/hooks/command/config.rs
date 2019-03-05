use error::*;
use settings::ConfigType;
use wrapper::config;

#[derive(Clone, Debug, Deserialize)]
pub(super) struct CommandEntry {
    pub(super) command: String,
    pub(super) files: String,
}

impl Default for CommandEntry {
    fn default() -> Self {
        Self {
            command: String::new(),
            files: "**/*".to_owned(),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct ConfigEntry {
    pub(super) enabled: bool,
    #[serde(rename = "pre-push")]
    pub(super) pre_push: Vec<CommandEntry>,
}

impl Default for ConfigEntry {
    fn default() -> Self {
        Self {
            enabled: true,
            pre_push: Vec::new(),
        }
    }
}

impl ConfigType for ConfigEntry {
    fn set_default(config: &mut config::Config) -> Result<()> {
        // config.set_default("command.enabled", false)?;

        Ok(())
    }
}
