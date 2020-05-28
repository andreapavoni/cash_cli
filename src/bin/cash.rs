extern crate cash;

use cash::cli;
use cash::ledger::Ledger;

fn main() {
    if let Some(command) = cli::parse() {
        let mut my_ledger: Ledger = Ledger::new();

        command.run(&mut my_ledger);
    }
}
