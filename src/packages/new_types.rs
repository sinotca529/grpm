use serde::Deserialize;

#[derive(Hash, Eq, Clone, PartialEq, Debug, Deserialize)]
pub struct Repo(pub String);
