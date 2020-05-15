pub mod export;
pub mod import;
pub mod list;
pub mod record;
pub mod report;

use clap::ArgMatches;

pub trait Command {
    fn new(opts: &ArgMatches) -> Self;
    fn run(&self) -> ();
}
