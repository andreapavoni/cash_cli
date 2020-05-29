//! A command to import data from a file.

extern crate csv;
extern crate serde;

use chrono::NaiveDate;
use clap::ArgMatches;
use serde_derive::Deserialize;
use std::fs::File;

use super::Command;
use crate::ledger::Ledger;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Record {
    date: String,
    category: String,
    label: String,
    description: String,
    earning: String,
    expense: String,
}

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

        let file = File::open(&self.src).unwrap();

        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b';')
            .from_reader(file);

        for result in rdr.deserialize() {
            let record: Record = result.unwrap();

            let date: NaiveDate = NaiveDate::parse_from_str(&record.date, "%d/%m/%Y").unwrap();
            let amount: i32;

            if record.expense != "" {
                amount = (record.expense.replace(",", ".").parse::<f32>().unwrap() * 100.0) as i32;
                my_ledger.withdraw(date, amount, &record.category, &record.label);
            } else {
                amount = (record
                    .earning
                    .replace(".", "")
                    .replace(",", ".")
                    .parse::<f32>()
                    .unwrap()
                    * 100.0) as i32;

                my_ledger.deposit(date, amount, &record.category, &record.label);
            }
        }
        my_ledger
    }
}
