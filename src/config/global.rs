use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct GlobalConfig {
    pub servers: Vec<ServerConfig>,
}

#[derive(Deserialize)]
pub struct ServerConfig {
    /// Server name or alias, this will be used to refer to the server
    pub alias: String,

    /// The name the server is called in your SSH config
    pub host: String,

    /// The base directory on the server where all the files are stored (not necessarily _this_ directory)
    pub base_directory: PathBuf,
}
