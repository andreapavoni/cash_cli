//! Defines a trait for Commands to run after CLI options are parsed.

pub mod export;
pub mod import;
pub mod list;
pub mod record;
pub mod report;

use crate::ledger::Ledger;

use clap::ArgMatches;

pub trait Command {
    fn new(opts: &ArgMatches) -> Self
    where
        Self: Sized;
    fn run<'a>(&self, ledger: &'a mut Ledger) -> &'a Ledger;
}
