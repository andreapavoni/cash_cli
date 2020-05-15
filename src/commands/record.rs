use super::Command;
use crate::storage::Storage;
use crate::wallet::Wallet;

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
    fn run(&self) {
        let mut my_wallet: Wallet = Storage::load_or_new("data.cbor".to_string());

        match self.operation.as_str() {
            "deposit" => my_wallet.deposit(
                self.date,
                self.amount,
                self.category.clone(),
                self.label.clone(),
            ),
            "withdraw" => my_wallet.withdraw(
                self.date,
                self.amount,
                self.category.clone(),
                self.label.clone(),
            ),
            _ => unreachable!(),
        }

        match Storage::save("data.cbor".to_string(), &my_wallet) {
            Ok(_) => (),
            Err(_e) => (),
        }
    }
}
