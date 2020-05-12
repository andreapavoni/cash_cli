use chrono::NaiveDate;
use std::env;

mod wallet;
use crate::wallet::Wallet;

mod analytics;
use crate::analytics::Analytics;

fn main() {
    let _config = resolve_configuration_file();

    let mut my_wallet = Wallet::new(String::from("default"), 0);

    println!("WALLET> {:?}", my_wallet);

    my_wallet.deposit(
        NaiveDate::parse_from_str("2020-05-05", "%Y-%m-%d").unwrap(),
        10_000,
        String::from("Entrate"),
        String::from("Stipendio"),
    );

    println!("WALLET> {:?}", my_wallet);

    my_wallet.withdraw(
        NaiveDate::parse_from_str("2020-05-05", "%Y-%m-%d").unwrap(),
        1000,
        String::from("Casa"),
        String::from("Spesa"),
    );

    println!("LEDGER> {:?}", my_wallet.ledger());

    let stats = Analytics::new(my_wallet.ledger());

    println!("STATS> {:?}", stats.categories());
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
