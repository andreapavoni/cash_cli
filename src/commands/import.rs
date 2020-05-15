//! A command to import data from a file.

use super::Command;
use crate::wallet::Wallet;

use clap::ArgMatches;

pub struct Import {
    src: String,
}

impl Command for Import {
    fn new(src: &ArgMatches) -> Import {
        Import {
            src: src.value_of("input").unwrap().to_string(),
        }
    }
    fn run<'a>(&self, my_wallet: &'a mut Wallet) -> &'a Wallet {
        println!("Import from file {:?}", self.src);
        my_wallet
    }
}
