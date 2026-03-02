use crate::commands::commands::Commands;
use clap::CommandFactory;
use clap::Parser;

mod commands;
mod config;
mod method;
mod server;

#[derive(Parser)]
#[command(
    name = "swhook",
    version,
    about,
    after_help = "Copyright (C) 2026  Illia <jandylokc@gmail.com>\nLicense GPL-3.0-or-later",
    arg_required_else_help = false
)]
pub struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

fn main() {
    match Cli::parse().command {
        Commands::Server => crate::commands::server::server(),
        Commands::Stop => crate::commands::stop::stop(),
        Commands::Reload => crate::commands::reload::reload(),
        Commands::Completions { shell } => {
            clap_complete::generate(shell, &mut Cli::command(), "swhook", &mut std::io::stdout())
        }
    }
}
