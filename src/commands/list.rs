//! A command to list operations.

use super::Command;
use crate::ledger::Ledger;

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

    fn run<'a>(&self, my_ledger: &'a mut Ledger) -> &'a Ledger {
        let mut filter_category: Option<String> = None;

        match &self.category {
            Some(category) => {
                println!(
                    "List operations for category {:?} on month {:?} in year {:?}:\n",
                    category, self.month, self.year
                );

                filter_category = Some(category.to_string());
            }
            None => {
                println!(
                    "List operations on month {:?} in year {:?}:\n",
                    self.month, self.year
                );
            }
        }

        for record in my_ledger.list_records(self.month, self.year, filter_category) {
            println!("{}", record);
        }

        my_ledger
    }
}
