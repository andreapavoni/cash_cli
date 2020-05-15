use super::Command;

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

    fn run(&self) {
        println!(
            "Report operations for year {:?} and month {:?} and category {:?}",
            self.year, self.month, self.category
        );
    }
}
