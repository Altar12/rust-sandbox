/*
cargo install cargo-edit
cargo add serde serde_derive serde_json ureq
*/
/*
fn main() {
    let responce = ureq::get("https://api.coinbase.com/v2/currencies")
        .call()
        .unwrap()
        .into_string()
        .unwrap();
    // Result|Option
    let coin_base_price: CoinsbasePrices = serde_json::from_str(responce.as_str()).unwrap();
    dbg!(coin_base_price);
}

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]

struct CoinsPrices {
    id: String,
    name: String,
    min_size: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct CoinsbasePrices {
    data: Vec<CoinsPrices>,
}
*/
