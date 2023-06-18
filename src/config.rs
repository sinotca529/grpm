use std::{fs::File, io::Read, path::PathBuf};
use anyhow::{Context, bail};
use directories_next::BaseDirs;
use serde::Deserialize;
use crate::packages::Packages;

const CONFIG_LOCATION: &'static str = ".config/grpm/grpm.toml";

#[derive(Hash, Eq, PartialEq, Debug, Deserialize)]
pub struct Config {
    install_dir: PathBuf,
    packages: Packages,
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        let config_path = BaseDirs::new()
            .context("Cannot get home directory.")?
            .home_dir()
            .join(CONFIG_LOCATION);

        if !config_path.exists() {
            bail!("Config file does not existing.");
        }

        let mut f = File::open(config_path)?;

        let mut contents = String::new();
        f.read_to_string(&mut contents)?;
        toml::from_str(&contents).context("Failed to parse config file")
    }

    pub fn install_dir(&self) -> &PathBuf {
        &self.install_dir
    }

    pub fn packages(&self) -> &Packages {
        &self.packages
    }
}

