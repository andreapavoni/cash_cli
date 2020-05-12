use super::*;

#[test]
fn withdraws_money_from_wallet() {
    let mut my_wallet = Wallet::new(String::from("default"), 10_000);
    assert_eq!(10_000, my_wallet.balance());

    my_wallet.withdraw(
        NaiveDate::parse_from_str("2020-05-05", "%Y-%m-%d").unwrap(),
        1000,
        String::from("Casa"),
        String::from("Spesa"),
    );

    assert_eq!(9000, my_wallet.balance())
}

#[test]
fn deposits_money_to_wallet() {
    let mut my_wallet = Wallet::new(String::from("default"), 0);
    assert_eq!(0, my_wallet.balance());

    my_wallet.deposit(
        NaiveDate::parse_from_str("2020-05-05", "%Y-%m-%d").unwrap(),
        1000,
        String::from("Casa"),
        String::from("Spesa"),
    );

    assert_eq!(1000, my_wallet.balance())
}
