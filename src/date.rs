pub use chrono::NaiveDate as Date;

#[must_use]
pub fn today() -> Date {
    chrono::offset::Local::now().date_naive()
}
