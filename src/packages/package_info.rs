use std::{fs::File, io::Read};

use anyhow::{Context, bail};
use regex::Regex;
use reqwest::blocking;
use serde::Deserialize;
use crate::config::Config;

use super::new_types::Repo;

const STATUS_LOCATION: &'static str = "grpm-stat.toml";

#[derive(Hash, Eq, PartialEq, Debug, Deserialize)]
pub struct PackageInfo {
    repo: Repo,
    tag_name: String,
    published_at: String,
}

impl PackageInfo {
    pub fn fetch_latest(repo: Repo) -> anyhow::Result<Self> {
        let url = format!("https://api.github.com/repos/{}/releases/latest", repo.0);

        let client: blocking::Client = blocking::Client::builder()
            .user_agent("request")
            .build()
            .context("Failed to make reqwest::blocking::Client")?;

        let response = client
            .get(&url)
            .send()
            .with_context(|| format!("Failed to GET {}", &url))?;

        if !response.status().is_success() {
            bail!("Failed to GET {}", &url);
        }

        let response_text = response
            .text()
            .with_context(|| format!("Failed to decode response of {}", &url))?;

        let published_at_pat = Regex::new(r#""published_at": "(.+)""#).unwrap();
        let Some(caps) = published_at_pat.captures(&response_text) else {
            bail!("Failed to get `published_at` of {}", repo.0);
        };
        let published_at = caps[1].to_string();

        let tag_name_pat = Regex::new(r#""tag_name": "(.+)""#).unwrap();
        let Some(caps) = tag_name_pat.captures(&response_text) else {
            bail!("Failed to get `tag_name` of {}", repo.0);
        };
        let tag_name = caps[1].to_string();

        Ok(Self {
            repo,
            published_at,
            tag_name,
        })
    }

    pub fn load_current_info(conf: &Config) -> anyhow::Result<Vec<Self>> {
        let stat_path = conf.install_dir().join(STATUS_LOCATION);

        if !stat_path.exists() {
            return Ok(vec![]);
        }

        let mut f = File::open(stat_path)?;
        let mut contents = String::new();
        f.read_to_string(&mut contents)?;

        toml::from_str(&contents).context("Failed to parse state file")
            .context("Failed to parse state file.")
    }
}
