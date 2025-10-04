use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about)]
#[command(propagate_version = true)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create an empty site config file in the current directory
    Init,

    /// Push the contents of a site to its configured server
    Push,
}
