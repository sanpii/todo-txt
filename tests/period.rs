extern crate chrono;
extern crate todo_txt;

#[cfg(feature="extended")]
use todo_txt::task::Period::*;

#[test]
#[cfg(feature="extended")]
fn add_year() {
    let current = Year + ::chrono::NaiveDate::from_ymd(1999, 1, 1);
    let expected = ::chrono::NaiveDate::from_ymd(2000, 1, 1);

    assert_eq!(current, expected);
}

#[test]
#[cfg(feature="extended")]
fn add_month() {
    let current = Month + ::chrono::NaiveDate::from_ymd(1999, 1, 1);
    let expected = ::chrono::NaiveDate::from_ymd(1999, 2, 1);

    assert_eq!(current, expected);
}

#[test]
#[cfg(feature="extended")]
fn add_month_extra() {
    let current = Month + ::chrono::NaiveDate::from_ymd(1999, 12, 1);
    let expected = ::chrono::NaiveDate::from_ymd(2000, 1, 1);

    assert_eq!(current, expected);
}

#[test]
#[cfg(feature="extended")]
fn add_week() {
    let current = Week + ::chrono::NaiveDate::from_ymd(1999, 1, 1);
    let expected = ::chrono::NaiveDate::from_ymd(1999, 1, 8);

    assert_eq!(current, expected);
}

#[test]
#[cfg(feature="extended")]
fn add_day() {
    let current = Day + ::chrono::NaiveDate::from_ymd(1999, 1, 1);
    let expected = ::chrono::NaiveDate::from_ymd(1999, 1, 2);

    assert_eq!(current, expected);
}

#[test]
#[cfg(feature="extended")]
fn add_day_extra() {
    let current = Day + ::chrono::NaiveDate::from_ymd(1999, 1, 31);
    let expected = ::chrono::NaiveDate::from_ymd(1999, 2, 1);

    assert_eq!(current, expected);

    let current = Day + ::chrono::NaiveDate::from_ymd(1999, 4, 30);
    let expected = ::chrono::NaiveDate::from_ymd(1999, 5, 1);

    assert_eq!(current, expected);

    let current = Day + ::chrono::NaiveDate::from_ymd(1999, 2, 28);
    let expected = ::chrono::NaiveDate::from_ymd(1999, 3, 1);

    assert_eq!(current, expected);

    let current = Day + ::chrono::NaiveDate::from_ymd(2000, 2, 28);
    let expected = ::chrono::NaiveDate::from_ymd(2000, 2, 29);

    assert_eq!(current, expected);

    let current = Day + ::chrono::NaiveDate::from_ymd(2000, 2, 29);
    let expected = ::chrono::NaiveDate::from_ymd(2000, 3, 1);

    assert_eq!(current, expected);
}
