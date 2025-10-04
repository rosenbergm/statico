use crate::placeholders;
use anyhow::anyhow;
use std::{env, fs};

pub fn init() -> anyhow::Result<()> {
    let cwd =
        env::current_dir().map_err(|error| anyhow!("Could not get current directory: {error}"))?;
    let config_path = cwd.join("statico_template.toml");

    if config_path.exists() {
        return Err(anyhow!(
            "A site config file already exists in this directory"
        ));
    }

    fs::write(config_path, placeholders::EMPTY_SITE_CONFIG)
        .map_err(|error| anyhow!("Could not write the initial config file: {error}"))?;

    Ok(())
}
