//! Defines a trait for Commands to run after CLI options are parsed.

pub mod export;
pub mod import;
pub mod list;
pub mod record;
pub mod report;

use crate::wallet::Wallet;

use clap::ArgMatches;

pub trait Command {
    fn new(opts: &ArgMatches) -> Self
    where
        Self: Sized;
    fn run<'a>(&self, wallet: &'a mut Wallet) -> &'a Wallet;
}
