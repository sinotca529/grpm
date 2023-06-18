use serde::Deserialize;
use super::{package_info::PackageInfo, new_types::Repo};

#[derive(Hash, Eq, PartialEq, Debug, Deserialize)]
pub struct Package {
    name: String,
    repo: Repo,
    bin: String,
    sign: Option<String>,
}

impl Package {
    fn update(&self) {}
    fn fetch_latest(&self) {}

    pub fn get_latest_package_info(&self) -> anyhow::Result<PackageInfo> {
        PackageInfo::fetch_latest(self.repo.clone())
    }
}

