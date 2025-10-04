#![deny(
    clippy::expect_used,
    clippy::future_not_send,
    clippy::pedantic,
    clippy::as_conversions,
    clippy::unwrap_used,
    unsafe_code
)]
#![allow(
    clippy::module_name_repetitions,
    clippy::missing_errors_doc,
    clippy::manual_non_exhaustive
)]

pub use args::{Args, Commands};
use clap::Parser;
pub use config::{GlobalConfig, SiteConfig};

mod args;
mod commands;
mod config;
mod placeholders;

fn run(command: &Commands) -> anyhow::Result<()> {
    match command {
        Commands::Init => commands::init(),
        Commands::Push => commands::push(),
    }
}

fn main() -> anyhow::Result<()> {
    let command = Args::parse();

    run(&command.command).inspect_err(|error| eprintln!("{error}"))
}
