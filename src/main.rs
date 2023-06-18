use crate::config::Config;

mod packages;
mod config;


fn main() -> anyhow::Result<()> {
    let cfg = Config::load()?;
    dbg!(cfg);

    Ok(())
}
