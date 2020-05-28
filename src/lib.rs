#[macro_use]
extern crate diesel;

// modules declarations
mod analytics;
mod commands;
mod db;
mod types;
mod utils;

pub mod cli;
pub mod ledger;

// fn main() {
//     if let Some(command) = cli::parse() {
//         let mut my_ledger: Ledger = Ledger::new();

//         command.run(&mut my_ledger);
//     }

// let _config = resolve_configuration_file();
// }

// Resolves the location of the configuration file
// fn resolve_configuration_file() -> String {
//     let mut home = env::var("HOME").unwrap();
//     home.push_str("/.config/cash.toml");

//     match env::var("XDG_CONFIG_HOME") {
//         Ok(path) => path,
//         _ => home,
//     }
// }
