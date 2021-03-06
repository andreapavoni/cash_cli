//! A command to print a report with analytics about operations.

use super::Command;
use crate::analytics::Analytics;
use crate::ledger::Ledger;
use crate::utils::format_money;
use std::collections::HashMap;

use clap::ArgMatches;

use cli_table::{
    format::{CellFormat, Justify, Padding},
    Cell, Row, Table,
};

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
        let table;

        if let Some(category) = &self.category {
            table = Table::new(
                build_rows(stats.labels(category.clone())),
                Default::default(),
            )
            .unwrap();
        } else {
            table = Table::new(build_rows(stats.categories()), Default::default()).unwrap();
        }

        table.print_stdout().unwrap();
        my_ledger
    }
}

fn build_rows(data: HashMap<String, i32>) -> Vec<Row> {
    let bold = CellFormat::builder().bold(true).build();
    let justify_right: CellFormat = CellFormat::builder()
        .justify(Justify::Right)
        .padding(Padding::builder().right(2).build())
        .build();
    let mut rows = vec![];

    let total: i32 = data.iter().fold(0i32, |sum, (_, amount)| sum + amount);

    for item in data {
        rows.push(build_row(item, justify_right))
    }

    rows.push(Row::new(vec![
        Cell::new("", Default::default()),
        Cell::new("", Default::default()),
    ]));

    rows.push(Row::new(vec![
        Cell::new("Total", bold),
        Cell::new(&format!("{}", format_money(total as i64)), justify_right),
    ]));

    rows
}

fn build_row(item: (String, i32), cell_format: CellFormat) -> Row {
    let (label, amount) = item;

    Row::new(vec![
        Cell::new(&label, Default::default()),
        Cell::new(&format!("{}", format_money(amount as i64)), cell_format),
    ])
}
