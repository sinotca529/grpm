use std::collections::HashMap;

use super::{new_types::Repo, package_info::PackageInfo};

struct PackageInfos(HashMap<Repo, PackageInfo>);

impl PackageInfos {
    fn check_state(current: &Self, latest: &Self) {
        for (repo, info) in current.0.iter() {
            match latest.0.get(repo) {
                Some(_) => todo!(),
                None => todo!(),
            }
        }
    }
}
