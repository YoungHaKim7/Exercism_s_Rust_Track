use time::Date;
use time::PrimitiveDateTime as DateTime;
use time_macros::{date, datetime, time};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start + time::Duration::seconds(100)
}

fn main() {
    let now = DateTime::new(date!(2020 - 01 - 01), time!(0:00));
    let gigasecond_later = after(now);
    println!("One gigasecond after now is {}", gigasecond_later);
}
