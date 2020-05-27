//! A command to print a report with analytics about operations.

use super::Command;
use crate::analytics::Analytics;
use crate::ledger::Ledger;

use clap::ArgMatches;

pub struct Report {
    month: u32,
    year: i32,
    category: Option<String>,
}

impl Command for Report {
    fn new(report: &ArgMatches) -> Report {
        let month: u32 = report.value_of("month").unwrap().parse::<u32>().unwrap();
        let year: i32 = report.value_of("year").unwrap().parse::<i32>().unwrap();

        if let Some(category) = report.value_of("category") {
            Report {
                month,
                year,
                category: Some(category.to_string()),
            }
        } else {
            Report {
                month,
                year,
                category: None,
            }
        }
    }

    fn run<'a>(&self, my_ledger: &'a mut Ledger) -> &'a Ledger {
        let mut filter_category: Option<String> = None;

        if let Some(category) = &self.category {
            filter_category = Some(category.to_string());
        }

        let records = my_ledger.list_records(self.month, self.year, filter_category);
        let stats = Analytics::new(&records);

        if let Some(category) = &self.category {
            println!(
                "Analytics for labels of category {:?} on month {:?} in year {:?}:\n",
                category, self.month, self.year
            );

            for (label, amount) in stats.labels(category.clone()) {
                println!("{:?} = {:?}", label, amount);
            }
        } else {
            println!(
                "Analytics for all categories on month {:?} in year {:?}:\n",
                self.month, self.year
            );

            for (category, amount) in stats.categories() {
                println!("{:?} = {:?}", category, amount);
            }
        }

        my_ledger
    }
}
