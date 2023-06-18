// use std::collections::HashMap;
// use super::new_types::Repo;
// use crate::{config::Config, packages::package_info::PackageInfo};
//
// enum SuggestedOpKind {
//     Update,
//     Remove,
//     Nop,
// }
//
// pub struct State(HashMap<Repo, SuggestedOpKind>);
//
// impl State {
//     fn new(conf: &Config) -> Self {
//         let current_info = PackageInfo::load_current_info(conf);
//         let latest_info = conf.packages().latest_info();
//         unimplemented!()
//     }
// }
