pub use chrono::NaiveDate as Date;

pub fn today() -> Date {
    chrono::offset::Local::today().naive_local()
}
