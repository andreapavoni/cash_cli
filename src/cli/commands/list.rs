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
                filter_category = Some(category.to_string());
            }
            None => {}
        }

        let records = my_ledger.list_records(self.month, self.year, filter_category);
        let table = Table::new(build_rows(records), Default::default()).unwrap();

        table.print_stdout().unwrap();

        my_ledger
    }
}

fn build_rows(records: Vec<ModelRecord>) -> Vec<Row> {
    let bold = CellFormat::builder().bold(true).build();
    let justify_right = CellFormat::builder()
        .justify(Justify::Right)
        .padding(Padding::builder().right(2).build())
        .build();

    let mut rows = vec![];

    rows.push(Row::new(vec![
        Cell::new("Date", bold),
        Cell::new("Operation", bold),
        Cell::new("Category", bold),
        Cell::new("Label", bold),
        Cell::new("", bold),
    ]));

    let expenses: i32 = records
        .iter()
        .filter(|item| item.operation == "withdraw")
        .fold(0i32, |sum, item| sum + item.amount);

    let earnings: i32 = records
        .iter()
        .filter(|item| item.operation == "deposit")
        .fold(0i32, |sum, item| sum + item.amount);

    for record in records {
        rows.push(build_row(record))
    }

    rows.push(Row::new(vec![
        Cell::new("", Default::default()),
        Cell::new("", Default::default()),
        Cell::new("", Default::default()),
        Cell::new("", Default::default()),
        Cell::new("", Default::default()),
    ]));

    rows.push(Row::new(vec![
        Cell::new("Total earnings", bold),
        Cell::new("", Default::default()),
        Cell::new("", Default::default()),
        Cell::new("", Default::default()),
        Cell::new(&format!("{}", format_money(earnings as i64)), justify_right),
    ]));

    rows.push(Row::new(vec![
        Cell::new("Total expenses", bold),
        Cell::new("", Default::default()),
        Cell::new("", Default::default()),
        Cell::new("", Default::default()),
        Cell::new(&format!("{}", format_money(expenses as i64)), justify_right),
    ]));

    rows.push(Row::new(vec![
        Cell::new("Total", bold),
        Cell::new("", Default::default()),
        Cell::new("", Default::default()),
        Cell::new("", Default::default()),
        Cell::new(
            &format!("{}", format_money((earnings - expenses) as i64)),
            justify_right,
        ),
    ]));

    rows
}

fn build_row(record: ModelRecord) -> Row {
    let justify_right = CellFormat::builder()
        .justify(Justify::Right)
        .padding(Padding::builder().right(2).build())
        .build();

    Row::new(vec![
        Cell::new(&format!("{}", record.date), Default::default()),
        Cell::new(&record.operation, Default::default()),
        Cell::new(&record.category, Default::default()),
        Cell::new(&record.label, Default::default()),
        Cell::new(
            &format!("{}", format_money(record.amount as i64)),
            justify_right,
        ),
    ])
}
