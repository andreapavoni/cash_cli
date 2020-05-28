use steel_cent::currency::EUR;
use steel_cent::formatting::france_style;
use steel_cent::Money;

pub fn format_money(amount: i64) -> String {
    let money = Money::of_minor(EUR, amount);
    format!("{}", france_style().display_for(&money))
}
