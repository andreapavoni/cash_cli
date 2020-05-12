use super::*;
use crate::wallet::Wallet;
use chrono::NaiveDate;
use std::collections::HashMap;

#[test]
fn calculates_stats_for_categories() {
    let mut my_wallet = Wallet::new(String::from("default"), 10_000);

    my_wallet.withdraw(
        NaiveDate::parse_from_str("2020-05-05", "%Y-%m-%d").unwrap(),
        1000,
        String::from("Casa"),
        String::from("Spesa"),
    );

    let mut expected: HashMap<String, i32> = HashMap::new();
    expected.insert(String::from("Casa"), 1000);

    let stats = Analytics::new(my_wallet.ledger());

    assert_eq!(expected, stats.categories())
}

#[test]
fn calculates_stats_for_category_labels() {
    let mut my_wallet = Wallet::new(String::from("default"), 10_000);

    my_wallet.withdraw(
        NaiveDate::parse_from_str("2020-05-05", "%Y-%m-%d").unwrap(),
        1000,
        String::from("Casa"),
        String::from("Spesa"),
    );

    let mut expected: HashMap<String, i32> = HashMap::new();
    expected.insert(String::from("Spesa"), 1000);

    let stats = Analytics::new(my_wallet.ledger());

    assert_eq!(expected, stats.labels(String::from("Casa")))
}
