//! A command to list operations.

use super::Command;
use crate::wallet::Wallet;

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

    fn run<'a>(&self, my_wallet: &'a mut Wallet) -> &'a Wallet {
        if let Some(category) = &self.category {
            println!(
                "List operations of category {:?} on month {:?} in year {:?}:\n",
                category, self.month, self.year
            );

            for operation in my_wallet
                .ledger()
                .into_iter()
                .filter(|&o| &o.category == category)
                .collect::<Vec<_>>()
            {
                println!("{:?}", operation);
            }
        } else {
            println!(
                "List operations on month {:?} in year {:?}:\n",
                self.month, self.year
            );

            for operation in my_wallet.ledger() {
                println!("{:?}", operation);
            }
        }

        my_wallet
    }
}
