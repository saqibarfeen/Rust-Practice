fn main() {
    println!("Hello, world!");
    calculateDaysBetweenDates(
        NaiveDate::from_ymd(2019, 1, 1),
        NaiveDate::from_ymd(2019, 1, 2),
    );
}

fn calculateDaysBetweenDates(begin, end) {
    let days = end - begin;
    println!("There are {} days between {} and {}", days, begin, end);
}
