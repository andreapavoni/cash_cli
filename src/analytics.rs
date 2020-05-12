//! Analytics
//! It implements logic to analyze the ledger.

use crate::wallet::Operation;
use std::collections::HashMap;
#[derive(Debug)]
pub struct Analytics<'a> {
    ledger: &'a Vec<Operation>,
}

impl Analytics<'_> {
    pub fn new<'a>(ledger: &'a Vec<Operation>) -> Analytics {
        Analytics { ledger }
    }

    /// Returns stats about categories
    pub fn categories(&self) -> HashMap<String, i32> {
        let mut stats: HashMap<String, i32> = HashMap::new();

        for op in self.ledger {
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

        for op in self.ledger {
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
