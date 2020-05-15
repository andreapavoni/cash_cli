//! A command to export data to a file.

use super::Command;
use crate::wallet::Wallet;

use clap::ArgMatches;
pub struct Export {
    dest: String,
}

impl Command for Export {
    fn new(dest: &ArgMatches) -> Export {
        Export {
            dest: dest.value_of("output").unwrap().to_string(),
        }
    }

    fn run<'a>(&self, my_wallet: &'a mut Wallet) -> &'a Wallet {
        println!("Export to file {:?}", self.dest);

        my_wallet
    }
}
