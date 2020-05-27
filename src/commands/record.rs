//! A command to record a new operation.

use super::Command;
use crate::ledger::Ledger;

use chrono::NaiveDate;
use clap::ArgMatches;

pub struct Record {
    operation: String,
    amount: i32,
    date: NaiveDate,
    category: String,
    label: String,
}

impl Command for Record {
    fn new(record: &ArgMatches) -> Record {
        Record {
            operation: record.value_of("operation").unwrap().to_string(),
            amount: record.value_of("amount").unwrap().parse::<i32>().unwrap(),
            date: NaiveDate::parse_from_str(record.value_of("date").unwrap(), "%Y-%m-%d").unwrap(),
            category: record.value_of("category").unwrap().to_string(),
            label: record.value_of("label").unwrap().to_string(),
        }
    }

    fn run<'a>(&self, my_ledger: &'a mut Ledger) -> &'a Ledger {
        match &self.operation[..] {
            "withdraw" => my_ledger.withdraw(
                self.date,
                self.amount,
                self.category.as_str(),
                self.label.as_str(),
            ),
            "deposit" => my_ledger.deposit(
                self.date,
                self.amount,
                self.category.as_str(),
                self.label.as_str(),
            ),
            _ => unreachable!(),
        };

        my_ledger
    }
}
