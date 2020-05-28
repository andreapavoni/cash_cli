//! A command to list operations.

use super::Command;
use crate::db::models::Record as ModelRecord;
use crate::ledger::Ledger;
use crate::utils::format_money;

use clap::ArgMatches;

use cli_table::{
    format::{CellFormat, Justify, Padding},
    Cell, Row, Table,
};

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

        let records = my_ledger.list_records(self.month, self.year, filter_category);
        let table = Table::new(build_rows(records), Default::default()).unwrap();

        table.print_stdout().unwrap();

        my_ledger
    }
}

fn build_rows(records: Vec<ModelRecord>) -> Vec<Row> {
    let bold = CellFormat::builder().bold(true).build();
    let mut rows = vec![];

    rows.push(Row::new(vec![
        Cell::new("Date", bold),
        Cell::new("Operation", bold),
        Cell::new("Category", bold),
        Cell::new("Label", bold),
        Cell::new("Amount", bold),
    ]));

    for record in records {
        rows.push(build_row(record))
    }

    rows
}

fn build_row(record: ModelRecord) -> Row {
    let justify_right = CellFormat::builder()
        .justify(Justify::Right)
        .padding(Padding::builder().right(2).build())
        .build();

    Row::new(vec![
        Cell::new(&format!("{}", record.date), justify_right),
        Cell::new(&record.operation, justify_right),
        Cell::new(&record.category, justify_right),
        Cell::new(&record.label, justify_right),
        Cell::new(
            &format!("{}", format_money(record.amount as i64)),
            justify_right,
        ),
    ])
}
