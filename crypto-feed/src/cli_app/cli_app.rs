use clap::{Args, Subcommand};
use eyre::Result;

use crate::binance::binance::Binance;

pub struct AppCli {}

impl AppCli {
  pub fn new() -> Self {
    AppCli {}
  }

  pub async fn execute(&self, subcommand: CliCommand) -> Result<()> {
    match subcommand.exchange {
      Exchange::Binance(bnc_subcommand) => {
        let binance = Binance::new();
        match bnc_subcommand.command {
          BinanceSubCommand::Stream(the_symbol) => {
            binance.stream(the_symbol.symbol)
          },
          BinanceSubCommand::Symbols => {
            let _ = binance.list_symbols().await;
          },
        }
      },
      _ => todo!(),
    }
    Ok(())
  }
}

#[derive(Args, Clone, Debug)]
pub struct CliCommand {
  #[clap(subcommand)]
  pub exchange: Exchange,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Exchange {
  /// Binance exchange
  Binance(BinanceCommand),
  /// Other exchange (not yet implemented)
  Other,
}

#[derive(Debug, Args, Clone)]
pub struct BinanceCommand {
  #[clap(subcommand)]
  pub command: BinanceSubCommand,
}

#[derive(Subcommand, Clone, Debug)]
pub enum BinanceSubCommand {
  /// Start listening the streaming
  Stream(Symbol),
  /// Show all the posible symbols
  Symbols,
}

#[derive(Clone, Args, Debug)]
pub struct Symbol {
  #[clap(short, long)]
  /// The symbol you want to operated with
  pub symbol: String,
}
