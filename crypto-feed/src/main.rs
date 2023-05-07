use args::{CalaArgs, Interface};
use clap::Parser;
use cli_app::cli_app::AppCli;
use dotenv::dotenv;
use std::{cell::RefCell, rc::Rc};
use tokio;
use tui_app::{app::App, crossterm_ui::start_ui};
mod args;
mod binance;
mod cli_app;
mod tui_app;

#[tokio::main]
async fn main() {
  dotenv().ok();
  let args = CalaArgs::parse();
  match args.execution {
    Interface::Cli(subcommand) => {
      let app = AppCli::new();
      let _ = app.execute(subcommand).await;
    },
    Interface::Tui => {
      let app = Rc::new(RefCell::new(App::new()));
      let _ = start_ui(app).await;
    },
  }
}
