mod new_types;
mod package;
mod package_info;
mod package_infos;
mod state;

use serde::Deserialize;
use crate::config::Config;

use self::{package::Package, package_info::PackageInfo};

#[derive(Hash, Eq, PartialEq, Debug, Deserialize)]
pub struct Packages(Vec<Package>);

impl Packages {
    fn latest_info(&self) -> anyhow::Result<Vec<PackageInfo>> {
        let mut v = Vec::with_capacity(self.0.len());
        for p in self.0.iter() {
            let info = p.get_latest_package_info()?;
            v.push(info);
        }
        Ok(v)
    }

    fn states(&self, conf: &Config) -> anyhow::Result<()> {
        let current_info = PackageInfo::load_current_info(conf)?;
        let latest_info = self.latest_info()?;

        unimplemented!()
    }
}
