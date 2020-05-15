use super::Command;

use clap::ArgMatches;

pub struct List {
    month: u32,
    year: i32,
    category: Option<String>,
}

impl Command for List {
    fn new(list: &ArgMatches) -> List {
        let month: u32 = list.value_of("month").unwrap().parse::<u32>().unwrap();
        let year: i32 = list.value_of("year").unwrap().parse::<i32>().unwrap();

        if let Some(category) = list.value_of("category") {
            List {
                month,
                year,
                category: Some(category.to_string()),
            }
        } else {
            List {
                month,
                year,
                category: None,
            }
        }
    }

    fn run(&self) {
        println!(
            "List operations for year {:?} and month {:?} and category {:?}",
            self.year, self.month, self.category
        );
    }
}
