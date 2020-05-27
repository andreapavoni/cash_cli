//! A command to import data from a file.

use super::Command;
use crate::ledger::Ledger;

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
    fn run<'a>(&self, my_ledger: &'a mut Ledger) -> &'a Ledger {
        println!("Import from file {:?}", self.src);
        my_ledger
    }
}
