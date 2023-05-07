use std::fmt::Display;

use reqwest;
use serde::{Deserialize, Serialize};
use tungstenite::connect;
use url::Url;

#[derive(Debug)]
pub struct Binance {
  api_base_url: String,
  stream_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Ticker {
  symbol: String,
  //#[serde(deserialize_with = "as_f64")]
  price: String,
}

impl Display for Ticker {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      serde_json::to_string_pretty(self).unwrap()
    )
  }
}

impl Binance {
  pub fn new() -> Self {
    let stream_url = std::env::var("binance_straem_base_url")
      .expect("Binance stream must be set");

    let api_base_url = std::env::var("binance_api_base_url")
      .expect("Binance api base url must be set");

    Binance {
      api_base_url: api_base_url,
      stream_url: stream_url,
    }
  }

  pub fn stream(&self, symbol: String) {
    let stream_url = self.stream_url.to_string() + "/" + &symbol;
    let stream_url = Url::parse(&stream_url).expect("Cannot parse stream url");

    let (mut socket, _) =
      connect(stream_url).expect("Cannot connect to the socket");

    loop {
      let msg = socket.read_message().expect("Error reading message");
      println!("Received: {}", msg);
    }
  }

  pub async fn list_symbols(&self) -> Vec<String> {
    let base_url = self.api_base_url.to_string() + "/ticker/price";
    let base_url = Url::parse(&base_url).expect("Cannot parse url.");

    let res = reqwest::get(base_url)
      .await
      .unwrap()
      .json::<Vec<Ticker>>()
      .await
      .unwrap();

    let symbols = res
      .iter()
      .map(|t| t.symbol.to_lowercase())
      .collect::<Vec<String>>();

    symbols
  }
}
