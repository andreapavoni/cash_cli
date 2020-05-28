use steel_cent::currency::EUR;
use steel_cent::formatting::france_style;
use steel_cent::Money;

pub fn format_money(amount: i64) -> String {
    // let euros = amount / 100;
    // let cents = amount % 100;
    // format!("Hello, world! {}.{}", euros, cents);

    let money = Money::of_minor(EUR, amount);
    format!("{}", france_style().display_for(&money))
}

// let price = Money::of_major_minor(USD, 19, 95);
// let shipping_and_handling = Money::of_major(USD, 10);
// let convenience_charge = Money::of_major(USD, 6);
// let fees = shipping_and_handling + convenience_charge;
// let discount: f64 = 1.0 - 0.2; // 20% off
// let discounted_price = price * discount;
// let total = discounted_price + fees;

// assert_eq!(Money::of_major_minor(USD, 15, 96), discounted_price);
// assert_eq!(Money::of_major_minor(USD, 31, 96), total);
// assert_eq!((price * discount) + shipping_and_handling + convenience_charge, total);

// assert_eq!("total: $31.96",
//            format!("total: {}", us_style().display_for(&total)));
// assert_eq!("total: USD31.96",
//            format!("total: {}", uk_style().display_for(&total)));
// assert_eq!("total: 31,96\u{a0}USD",
//            format!("total: {}", france_style().display_for(&total)));
