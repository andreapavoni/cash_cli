//! Wallet
//! It implements logic to handle a ledger with deposit/withdrawal operations.

use chrono::NaiveDate;
use serde_derive::{Deserialize, Serialize};

use crate::db;
use crate::db::models::NewRecord as ModelNewRecord;
use crate::db::models::Record as ModelRecord;
use crate::db::models::RecordFilter;

#[cfg(test)]
#[path = "test/ledger_test.rs"]
mod ledger_test;

/// Record type, it might either be a withdraw or a deposit.
#[derive(Debug, Serialize, Deserialize)]
enum OperationTypes {
    Withdraw,
    Deposit,
}

/// A ledger.
#[derive(Debug, Serialize, Deserialize)]
pub struct Ledger {}

impl Ledger {
    /// Creates a new ledger.
    pub fn new() -> Ledger {
        Ledger {}
    }

    /// Withdraws money.
    #[allow(dead_code)]
    pub fn withdraw(
        &mut self,
        date: NaiveDate,
        amount: i32,
        category: &str,
        label: &str,
        description: &str,
    ) -> &Self {
        self.apply_operation(
            OperationTypes::Withdraw,
            date,
            -amount,
            category,
            label,
            description,
        );

        self
    }

    /// Deposits money.
    #[allow(dead_code)]
    pub fn deposit(
        &mut self,
        date: NaiveDate,
        amount: i32,
        category: &str,
        label: &str,
        description: &str,
    ) -> &Self {
        self.apply_operation(
            OperationTypes::Deposit,
            date,
            amount,
            category,
            label,
            description,
        );
        self
    }

    #[allow(dead_code)]
    pub fn list_records(
        &self,
        month: u32,
        year: i32,
        category: Option<String>,
    ) -> Vec<ModelRecord> {
        let conn = db::build_conn();

        let filter: RecordFilter = RecordFilter {
            month,
            year,
            category,
        };

        ModelRecord::list(&conn, filter)
    }

    fn apply_operation(
        &mut self,
        ot: OperationTypes,
        date: NaiveDate,
        amount: i32,
        category: &str,
        label: &str,
        description: &str,
    ) -> &Ledger {
        let operation = match ot {
            OperationTypes::Deposit => "deposit",
            OperationTypes::Withdraw => "withdraw",
        };

        let conn = db::build_conn();
        let model_record = ModelNewRecord {
            operation,
            amount,
            date,
            category,
            label,
            description,
        };

        ModelRecord::insert(&conn, &model_record);

        self
    }
}
