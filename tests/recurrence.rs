extern crate chrono;
extern crate todo_txt;

#[cfg(all(test, feature = "extended"))]
mod test {
    use chrono::NaiveDate;
    use std::str::FromStr;
    use todo_txt::task::Recurrence;

    #[test]
    fn from_invalid() {
        use std::str::FromStr;

        assert_eq!(Recurrence::from_str("1"), Err(()));
    }

    #[test]
    fn add_years() {
        let current = Recurrence::from_str("4y").unwrap() + NaiveDate::from_ymd(1998, 1, 1);
        let expected = NaiveDate::from_ymd(2002, 1, 1);

        assert_eq!(current, expected);
    }

    #[test]
    fn add_months() {
        let current = Recurrence::from_str("4m").unwrap() + NaiveDate::from_ymd(1999, 11, 1);
        let expected = NaiveDate::from_ymd(2000, 3, 1);

        assert_eq!(current, expected);
    }

    #[test]
    fn add_months_extra() {
        let current = Recurrence::from_str("2m").unwrap() + NaiveDate::from_ymd(2009, 12, 31);
        let expected = NaiveDate::from_ymd(2010, 2, 28);

        assert_eq!(current, expected);
    }

    #[test]
    fn add_months_sticky() {
        let current = Recurrence::from_str("3m").unwrap() + NaiveDate::from_ymd(2010, 2, 28);
        let expected = NaiveDate::from_ymd(2010, 5, 31);

        assert_eq!(current, expected);
    }
}
