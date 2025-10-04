use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct SiteConfig {
    /// The name of the site, this will be used to refer to the site
    pub name: String,

    /// The alias of the server where the site is hosted, this must match one of the servers in the global config.
    pub server: String,

    /// The directory where the site's files are going to be stored. This is appended to the `base_directory` of the server
    pub directory: String,

    /// The path from where to copy the files to the server.
    pub output_dir: PathBuf,
}
