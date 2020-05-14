// use clap::App;
use std::env;

// modules declarations
mod analytics;
mod storage;
mod types;
mod wallet;

use crate::analytics::Analytics;
use crate::storage::Storage;
use crate::wallet::Wallet;

fn main() {
    let _config = resolve_configuration_file();

    let my_wallet: Wallet = match Storage::load("data.cbor".to_string()) {
        Ok(w) => w,
        Err(_e) => Wallet::new(String::from("default"), 0),
    };

    println!("WALLET> {:?}", my_wallet);

    let categories_stats = Analytics::new(my_wallet.ledger());

    for category in categories_stats.categories().keys() {
        println!("@ {:?}", category);
        let labels_stats = Analytics::new(my_wallet.ledger());
        println!("==> {:?}", labels_stats.labels(category.clone()));
    }

    // NOTE: to remove this `match` syntax I _should_ change return value of the `main` func
    match Storage::save("data.cbor".to_string(), &my_wallet) {
        Ok(_) => (),
        Err(_e) => (),
    }
}

/// Resolves the location of the stup configuration file
fn resolve_configuration_file() -> String {
    let mut home = env::var("HOME").unwrap();
    home.push_str("/.config/cash.toml");

    match env::var("XDG_CONFIG_HOME") {
        Ok(path) => path,
        _ => home,
    }
}
