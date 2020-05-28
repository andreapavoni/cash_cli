use chrono::NaiveDate;
use std::fmt;

use super::schema::records;
extern crate diesel;
use self::diesel::prelude::*;

use crate::utils::format_money;

#[derive(Debug, Queryable)]
pub struct Record {
    pub id: i32,
    pub amount: i32,
    pub category: String,
    pub label: String,
    pub date: NaiveDate,
    pub operation: String,
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {} {} {}",
            self.date,
            self.operation,
            format_money(self.amount as i64),
            self.category,
            self.label
        )
    }
}

#[derive(Insertable)]
#[table_name = "records"]
pub struct NewRecord<'a> {
    pub date: NaiveDate,
    pub amount: i32,
    pub operation: &'a str,
    pub category: &'a str,
    pub label: &'a str,
}

pub struct RecordFilter {
    pub month: u32,
    pub year: i32,
    pub category: Option<String>,
}

impl Record {
    /// Run query using Diesel to insert a new database row and return the result.
    pub fn insert(conn: &SqliteConnection, new_record: &NewRecord) -> usize {
        diesel::insert_into(records::table)
            .values(new_record)
            .execute(conn)
            .expect("Error saving new record")
    }

    pub fn list(conn: &SqliteConnection, filter: RecordFilter) -> Vec<Record> {
        let start_date: NaiveDate = NaiveDate::from_ymd(filter.year, filter.month, 1);
        let end_date: NaiveDate = NaiveDate::from_ymd(filter.year, filter.month + 1, 1);

        let mut query = records::table.into_boxed();

        if let Some(category) = filter.category {
            query = query.filter(records::category.eq(category));
        }

        query
            .filter(records::date.ge(start_date).and(records::date.lt(end_date)))
            .order_by(records::date.asc())
            .load::<Record>(conn)
            .unwrap()
    }
}
