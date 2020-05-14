use chrono::NaiveDate;

use super::Command;

pub struct Record {
    operation: String,
    amount: u32,
    date: NaiveDate,
    category: String,
    label: String,
}

impl Record {
    pub fn new(
        operation: String,
        date: NaiveDate,
        amount: u32,
        category: String,
        label: String,
    ) -> Record {
        Record {
            operation,
            amount,
            date,
            category,
            label,
        }
    }
}

impl Command for Record {
    fn run(&self) {
        println!(
            "Operation {:?} passed with amount: {:?} on date {:?} for category {:?} and label {:?}",
            self.operation, self.amount, self.date, self.category, self.label
        );
    }
}
