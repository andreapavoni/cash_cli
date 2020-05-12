//! Wallet
//! It implements logic to handle a wallet with deposit/withdrawal operations, while also
//! keeping a ledger.

use chrono::NaiveDate;

/// Operation type, it might either be a withdraw or a deposit.
#[derive(Debug)]
pub enum OperationTypes {
    Withdraw,
    Deposit,
}

/// Defines a single operation.
#[derive(Debug)]
pub struct Operation {
    pub date: NaiveDate,
    pub amount: i32,
    pub operation: OperationTypes,
    pub category: String,
    pub label: String,
}

impl Operation {
    /// Creates a new Operation instance.
    pub fn new(
        operation: OperationTypes,
        date: NaiveDate,
        amount: i32,
        category: String,
        label: String,
    ) -> Operation {
        Operation {
            date,
            amount,
            operation,
            category,
            label,
        }
    }
}
/// A wallet.
#[derive(Debug)]
pub struct Wallet {
    name: String,
    balance: i32,
    ledger: Vec<Operation>,
}

impl Wallet {
    /// Creates a new wallet.
    pub fn new(name: String, balance: i32) -> Wallet {
        Wallet {
            name,
            balance,
            ledger: vec![],
        }
    }

    /// Withdraws money from wallet.
    pub fn withdraw(&mut self, date: NaiveDate, amount: i32, category: String, label: String) {
        self.apply_operation(OperationTypes::Withdraw, date, amount, category, label);
        ()
    }

    /// Deposits money into wallet.
    pub fn deposit(&mut self, date: NaiveDate, amount: i32, category: String, label: String) {
        self.apply_operation(OperationTypes::Deposit, date, amount, category, label);
        ()
    }

    /// Returns the list of operations happened on wallet.
    pub fn ledger(&self) -> &Vec<Operation> {
        &self.ledger
    }

    fn apply_operation(
        &mut self,
        ot: OperationTypes,
        date: NaiveDate,
        amount: i32,
        category: String,
        label: String,
    ) -> &Wallet {
        match ot {
            OperationTypes::Deposit => {
                self.balance += amount;
            }
            OperationTypes::Withdraw => {
                self.balance -= amount;
            }
        }

        let operation = Operation::new(ot, date, amount, category, label);
        self.ledger.push(operation);
        self
    }
}
