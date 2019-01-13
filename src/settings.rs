use error::*;
use std::path::PathBuf;
use wrapper::config;

pub(crate) trait ConfigType {
    fn set_default(_: &mut config::Config) -> Result<()>;

    fn set_values(_: &mut config::Config) -> Result<()> {
        Ok(())
    }
}

macro_rules! settings {
    ( $( $mod:ident ),* ) => {
        #[derive(Clone, Debug, Deserialize)]
        pub(crate) struct Settings {
            $(
                pub(crate) $mod: $mod::ConfigEntry,
            )*
        }

        impl Settings {
            pub(crate) fn init(
                mut home_path: PathBuf,
                mut git_root_path: PathBuf
            ) -> Result<Self> {
                let mut config = config::Config::new();

                $(
                    $mod::ConfigEntry::set_default(&mut config)?;
                )*

                home_path.push(".git-hooks.toml");
                git_root_path.push(".git-hooks.toml");
                config.set_path(home_path)?;
                config.set_path(git_root_path)?;

                $(
                    $mod::ConfigEntry::set_values(&mut config)?;
                )*

                config.try_into()
            }
        }
    }
}

settings!();
