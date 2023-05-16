#[cfg(all(test, feature = "extended"))]
mod test {
    use chrono::NaiveDate;
    use std::str::FromStr;
    use todo_txt::task::Recurrence;

    #[test]
    fn from_invalid() {
        use std::str::FromStr;

        for p in &["1", "+1"] {
            assert!(Recurrence::from_str(p).is_err());
        }
    }

    #[test]
    fn from_valid() {
        use std::str::FromStr;

        for p in &["+4y", "+4m", "+4w", "+4d", "12m"] {
            assert!(Recurrence::from_str(p).is_ok());
        }
    }

    #[test]
    fn add_years() {
        let current =
            Recurrence::from_str("4y").unwrap() + NaiveDate::from_ymd_opt(1998, 1, 1).unwrap();
        let expected = NaiveDate::from_ymd_opt(2002, 1, 1).unwrap();

        assert_eq!(current, expected);
    }

    #[test]
    fn add_months() {
        let current =
            Recurrence::from_str("4m").unwrap() + NaiveDate::from_ymd_opt(1999, 11, 1).unwrap();
        let expected = NaiveDate::from_ymd_opt(2000, 3, 1).unwrap();

        assert_eq!(current, expected);
    }

    #[test]
    fn add_months_extra() {
        let current =
            Recurrence::from_str("2m").unwrap() + NaiveDate::from_ymd_opt(2009, 12, 31).unwrap();
        let expected = NaiveDate::from_ymd_opt(2010, 2, 28).unwrap();

        assert_eq!(current, expected);
    }

    #[test]
    fn add_months_sticky() {
        let current =
            Recurrence::from_str("3m").unwrap() + NaiveDate::from_ymd_opt(2010, 2, 28).unwrap();
        let expected = NaiveDate::from_ymd_opt(2010, 5, 31).unwrap();

        assert_eq!(current, expected);
    }
}
