//! A module to parse CLI options.

use chrono::{Datelike, Local, NaiveDate};
use clap::{crate_authors, crate_version, App, Arg};

use crate::commands::{
    export::Export, import::Import, list::List, record::Record, report::Report, Command,
};

// Usage ideas:

// OPERATIONS:
// cash record withdraw --amount 1000 --date 2020-05-05 --category Fondamentali --label Spesa --notes Conad
// cash record deposit --amount 100000 --date 2020-05-05 --category Entrate --label Stipendio

// DATA & STATS:
// cash list --month 5 --year 2020
// cash list --month 5 --year 2020 --category Fondamentali
// cash report --month 5 --year 2020 --category Fondamentali

// IMPORT / EXPORT:
// cash import input.csv
// cash export output.csv --month 05 --year 2020

pub fn parse() -> Option<Box<dyn Command>> {
    let matches = App::new("Cash")
        .about("Simple personal finance manager")
        .author(crate_authors!())
        .version(crate_version!())
        .subcommand(
            App::new("record")
                .about("Registers a new operation") // The message displayed in "cash -h" or "cash help"
                .arg(
                    Arg::with_name("operation")
                        .about("Operation to apply")
                        .possible_values(&["withdraw", "deposit"])
                        .required(true),
                )
                .arg(
                    Arg::with_name("amount")
                        .about("Sets operation amount")
                        .takes_value(true)
                        .long("amount")
                        .short('a')
                        .validator(is_valid_amount)
                        .required(true),
                )
                .arg(
                    Arg::with_name("date")
                        .about("Sets date (YYYY-mm-dd)")
                        .takes_value(true)
                        .long("date")
                        .short('d')
                        .validator(is_valid_date)
                        .default_value(&Local::today().naive_local().to_string()),
                )
                .arg(
                    Arg::with_name("category")
                        .about("Sets category for operation")
                        .takes_value(true)
                        .long("category")
                        .short('c')
                        .required(true),
                )
                .arg(
                    Arg::with_name("label")
                        .about("Sets label for operation")
                        .takes_value(true)
                        .long("label")
                        .short('l')
                        .required(true),
                ),
        )
        .subcommand(
            App::new("list")
                .about("Lists operations")
                .arg(
                    Arg::with_name("month")
                        .about("Filters by month")
                        .takes_value(true)
                        .long("month")
                        .short('m')
                        .validator(is_valid_month)
                        .default_value(&current_month().to_string()),
                )
                .arg(
                    Arg::with_name("year")
                        .about("Filters by year")
                        .takes_value(true)
                        .long("year")
                        .short('y')
                        .validator(is_valid_year)
                        .default_value(&current_year().to_string()),
                )
                .arg(
                    Arg::with_name("category")
                        .about("Filters by category")
                        .takes_value(true)
                        .long("category")
                        .short('c'),
                ),
        )
        .subcommand(
            App::new("report")
                .about("Print a report")
                .arg(
                    Arg::with_name("month")
                        .about("Filters by month")
                        .takes_value(true)
                        .long("month")
                        .short('m')
                        .validator(is_valid_month)
                        .default_value(&current_month().to_string()),
                )
                .arg(
                    Arg::with_name("year")
                        .about("Filters by year")
                        .takes_value(true)
                        .long("year")
                        .short('y')
                        .validator(is_valid_year)
                        .default_value(&current_year().to_string()),
                )
                .arg(
                    Arg::with_name("category")
                        .about("Filters by category and shows labels")
                        .takes_value(true)
                        .long("category")
                        .short('c')
                        .required(true),
                ),
        )
        .subcommand(
            App::new("import")
                .about("Imports from a CSV file")
                .arg(Arg::with_name("input").about("path to CSV file to read")),
        )
        .subcommand(
            App::new("export")
                .about("Exports to a CSV file")
                .arg(Arg::with_name("output").about("path to CSV file to write")),
        )
        .get_matches();

    match matches.subcommand() {
        ("record", Some(record)) => Some(Box::new(Record::new(record))),
        ("list", Some(list)) => Some(Box::new(List::new(list))),
        ("report", Some(report)) => Some(Box::new(Report::new(report))),
        ("import", Some(src)) => Some(Box::new(Import::new(src))),
        ("export", Some(dest)) => Some(Box::new(Export::new(dest))),
        _ => None,
    }
}

fn is_valid_amount(amount: &str) -> Result<(), String> {
    match amount.to_string().parse::<i32>() {
        Ok(_) => Ok(()),
        Err(_e) => Err(String::from("invalid amount")),
    }
}

fn is_valid_date(val: &str) -> Result<(), String> {
    match NaiveDate::parse_from_str(val, "%Y-%m-%d") {
        Ok(_) => Ok(()),
        Err(_e) => Err(String::from("invalid date")),
    }
}

fn is_valid_month(month: &str) -> Result<(), String> {
    let month: i32 = if let Ok(num) = month.to_string().parse::<i32>() {
        num
    } else {
        0
    };

    if (1..12).contains(&month) {
        Ok(())
    } else {
        Err(String::from("invalid month"))
    }
}

fn is_valid_year(month: &str) -> Result<(), String> {
    match month.to_string().parse::<i32>() {
        Ok(_) => Ok(()),
        Err(_e) => Err(String::from("invalid month")),
    }
}

fn current_date() -> NaiveDate {
    Local::today().naive_local()
}

fn current_month() -> u32 {
    current_date().month()
}

fn current_year() -> i32 {
    current_date().year()
}
