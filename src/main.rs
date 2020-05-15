// use std::env;

// modules declarations
mod analytics;
mod cli_args;
mod commands;
mod storage;
mod types;
mod wallet;

use crate::cli_args::parse;
use crate::storage::Storage;
use crate::wallet::Wallet;

fn main() {
    if let Some(command) = parse() {
        let mut my_wallet: Wallet = Storage::load_or_new("data.cbor".to_string());

        command.run(&mut my_wallet);

        match Storage::save("data.cbor".to_string(), &my_wallet) {
            Ok(_) => (),
            Err(_e) => (),
        }
    }

    // let _config = resolve_configuration_file();
}

// Resolves the location of the configuration file
// fn resolve_configuration_file() -> String {
//     let mut home = env::var("HOME").unwrap();
//     home.push_str("/.config/cash.toml");

//     match env::var("XDG_CONFIG_HOME") {
//         Ok(path) => path,
//         _ => home,
//     }
// }
