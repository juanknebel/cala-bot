use clap::{Parser, Subcommand};

use crate::cli_app::cli_app::CliCommand;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CalaArgs {
  #[clap(subcommand)]
  pub execution: Interface,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Interface {
  /// Command line interface
  Cli(CliCommand),
  /// Terminal user interface
  Tui,
}
