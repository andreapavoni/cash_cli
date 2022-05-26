//! A command to import data from a file.

extern crate csv;
extern crate serde;

use chrono::{Datelike, NaiveDate};
use clap::ArgMatches;
use serde_derive::Deserialize;
use std::fs::File;

use super::Command;
use crate::cli::current_date;
use crate::ledger::Ledger;

#[derive(Debug, Deserialize)]
struct CsvRecord {
    date: String,
    category: String,
    label: String,
    description: String,
    earning: String,
    expense: String,
}

#[derive(Debug)]
struct ParsedRecord {
    date: NaiveDate,
    category: String,
    label: String,
    description: String,
    operation: String,
    amount: i32,
}

pub struct Import {
    src: String,
    is_template: bool,
}

impl Command for Import {
    fn new(args: &ArgMatches) -> Import {
        let is_template: bool = args.is_present("template");

        Import {
            src: args.value_of("input").unwrap().to_string(),
            is_template,
        }
    }
    fn run<'a>(&self, my_ledger: &'a mut Ledger) -> &'a Ledger {
        println!("Import from file {:?}", self.src);

        let records = parse_csv(&self.src);

        for record in &records {
            let date: NaiveDate = set_date(record.date, self.is_template);

            if record.operation == "withdraw" {
                my_ledger.withdraw(
                    date,
                    record.amount,
                    &record.category,
                    &record.label,
                    &record.description,
                );
            } else {
                my_ledger.deposit(
                    date,
                    record.amount,
                    &record.category,
                    &record.label,
                    &record.description,
                );
            }
        }

        my_ledger
    }
}

fn parse_csv(path: &str) -> Vec<ParsedRecord> {
    let mut records = vec![];
    let file = File::open(path).unwrap();

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b';')
        .from_reader(file);

    for result in rdr.deserialize() {
        let record: CsvRecord = result.unwrap();

        let date: NaiveDate = NaiveDate::parse_from_str(&record.date, "%d/%m/%Y").unwrap();

        let (operation, amount) = parse_operation(&record);
        let category = record.category;
        let label = record.label;
        let description = record.description;

        records.push(ParsedRecord {
            date,
            category,
            label,
            description,
            operation,
            amount,
        });
    }

    records
}

fn parse_amount(str_amount: &str) -> i32 {
    (str_amount
        .replace(".", "")
        .replace(",", ".")
        .parse::<f32>()
        .unwrap()
        * 100.0) as i32
}

fn parse_operation<'a>(record: &'a CsvRecord) -> (String, i32) {
    if record.expense != "" {
        (String::from("withdraw"), parse_amount(&record.expense))
    } else {
        (String::from("deposit"), parse_amount(&record.earning))
    }
}

fn set_date(date: NaiveDate, is_template: bool) -> NaiveDate {
    if is_template {
        let today = current_date();
        NaiveDate::from_ymd(today.year(), today.month(), date.day())
    } else {
        date
    }
}
