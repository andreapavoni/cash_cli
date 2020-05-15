use super::Command;

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

    fn run(&self) {
        println!("Export to file {:?}", self.dest);
    }
}
