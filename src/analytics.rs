//! Analytics
//! It implements logic to analyze the records.

use crate::db::models::Record;
use std::collections::HashMap;

#[cfg(test)]
#[path = "test/analytics_test.rs"]
mod analytics_test;

#[derive(Debug)]
pub struct Analytics<'a> {
    records: &'a Vec<Record>,
}

impl Analytics<'_> {
    pub fn new<'a>(records: &'a Vec<Record>) -> Analytics {
        Analytics { records }
    }

    /// Returns stats about categories
    pub fn categories(&self) -> HashMap<String, i32> {
        let mut stats: HashMap<String, i32> = HashMap::new();

        for op in self.records {
            let val = match stats.get(&op.category) {
                Some(v) => v + op.amount,
                _ => op.amount,
            };
            stats.insert(op.category.clone(), val);
        }
        stats
    }

    /// Returns stats about labels in a given category
    pub fn labels(&self, category: String) -> HashMap<String, i32> {
        let mut stats: HashMap<String, i32> = HashMap::new();

        for op in self.records {
            if op.category == category {
                let val = match stats.get(&op.label) {
                    Some(v) => v + op.amount,
                    _ => op.amount,
                };
                stats.insert(op.label.clone(), val);
            }
        }
        stats
    }
}
