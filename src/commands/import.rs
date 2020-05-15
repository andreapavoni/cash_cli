use super::Command;

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

    fn run(&self) {
        println!("Import from file {:?}", self.src);
    }
}
