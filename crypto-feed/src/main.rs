use args::{CalaArgs, Interface};
use clap::Parser;
use cli_app::cli_app::AppCli;
use dotenv::dotenv;
use rand;
use std::{cell::RefCell, rc::Rc};
use tokio;
mod args;
mod binance;
mod cli_app;

#[tokio::main]
async fn main() {
  let a = 4;

  let b = 66;
  let c = a * b;
  println!("{} * {} = {}", a, b, c);
  String::new();
  let s = 4;

  dotenv().ok();
  let args = CalaArgs::parse();
  match args.execution {
    Interface::Cli(subcommand) => {
      let app = AppCli::new();
      let _ = app.execute(subcommand).await;
    },
    Interface::Tui => {
      todo!()
    },
  }
}
