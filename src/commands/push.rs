use anyhow::anyhow;
use inquire::Confirm;
use std::{env, process::Command};

use crate::{GlobalConfig, SiteConfig};

pub fn push() -> anyhow::Result<()> {
    let cwd =
        env::current_dir().map_err(|error| anyhow!("Could not get current directory: {error}"))?;

    let global_config_path = dirs::config_dir()
        .ok_or_else(|| anyhow!("Could not get config directory"))?
        .join("statico")
        .join("config.toml");

    let site_config_path = env::current_dir()
        .map_err(|error| anyhow!("Could not get current directory: {error}"))?
        .join("statico.toml");

    let global_config: GlobalConfig = toml::from_str(
        std::fs::read_to_string(global_config_path)
            .map_err(|error| anyhow!("Could not read global config: {error}"))?
            .as_ref(),
    )
    .map_err(|error| anyhow!("Could not parse global config: {error}"))?;

    let site_config: SiteConfig = toml::from_str(
        std::fs::read_to_string(site_config_path)
            .map_err(|error| anyhow!("Could not read site config: {error}"))?
            .as_ref(),
    )
    .map_err(|error| anyhow!("Could not parse site config: {error}"))?;

    let server = global_config
        .servers
        .iter()
        .find(|s| s.alias == site_config.server)
        .ok_or_else(|| anyhow!("Could not find server in global config."))?;

    let build_dir = cwd.join(&site_config.output_dir);

    if !build_dir.exists() {
        return Err(anyhow!(
            "Build directory does not exist: {}",
            build_dir.display()
        ));
    }

    let final_path = server.base_directory.join(site_config.directory);

    #[expect(clippy::unwrap_used)]
    let files = glob::glob(format!("{}/*", build_dir.to_str().unwrap()).as_str()).unwrap();

    let mut file_strings = Vec::new();

    for file in files {
        let file = file.map_err(|error| anyhow!("Could not get file: {error}"))?;
        let path = file
            .to_str()
            .ok_or_else(|| anyhow!("Could not convert file path to string"))?
            .to_string();

        file_strings.push(path);
    }

    println!(
        "This will overwrite all data in {} and upload the following files:\n\n{}\n\n",
        final_path.to_string_lossy(),
        file_strings.join("\n")
    );

    let should_continue = Confirm::new("Are you sure you want to push the site?")
        .with_default(false)
        .with_help_message("This will overwrite all data in the target directory on the server.")
        .prompt()
        .map_err(|error| anyhow!("Something went wrong while inquiring the user: {error}"))?;

    if !should_continue {
        return Err(anyhow!("Aborted."));
    }

    let source = format!("{}/", build_dir.to_string_lossy());
    let destination = format!("{}:{}", server.host, final_path.to_string_lossy());

    let args = vec!["-avz", &source, &destination];

    let mut child = Command::new("rsync")
        .args(args)
        .spawn()
        .map_err(|error| anyhow!("Unable to run `rsync`: {error}"))?;

    println!("Working...");

    child
        .wait()
        .map_err(|error| anyhow!("`rsync` command failed: {error}"))?;

    println!("Done!");

    Ok(())
}
