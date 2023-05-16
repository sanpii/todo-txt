#[cfg(all(test, feature = "extended"))]
mod test {
    use todo_txt::task::Period::*;

    #[test]
    fn add_year() {
        let current = Year + chrono::NaiveDate::from_ymd_opt(1999, 1, 1).unwrap();
        let expected = chrono::NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();

        assert_eq!(current, expected);
    }

    #[test]
    fn add_month() {
        let current = Month + chrono::NaiveDate::from_ymd_opt(1999, 1, 1).unwrap();
        let expected = chrono::NaiveDate::from_ymd_opt(1999, 2, 1).unwrap();

        assert_eq!(current, expected);
    }

    #[test]
    fn add_month_extra() {
        let current = Month + chrono::NaiveDate::from_ymd_opt(1999, 12, 1).unwrap();
        let expected = chrono::NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();

        assert_eq!(current, expected);
    }

    #[test]
    fn add_week() {
        let current = Week + chrono::NaiveDate::from_ymd_opt(1999, 1, 1).unwrap();
        let expected = chrono::NaiveDate::from_ymd_opt(1999, 1, 8).unwrap();

        assert_eq!(current, expected);
    }

    #[test]
    fn add_day() {
        let current = Day + chrono::NaiveDate::from_ymd_opt(1999, 1, 1).unwrap();
        let expected = chrono::NaiveDate::from_ymd_opt(1999, 1, 2).unwrap();

        assert_eq!(current, expected);
    }

    #[test]
    fn add_day_extra() {
        let current = Day + chrono::NaiveDate::from_ymd_opt(1999, 1, 31).unwrap();
        let expected = chrono::NaiveDate::from_ymd_opt(1999, 2, 1).unwrap();

        assert_eq!(current, expected);

        let current = Day + chrono::NaiveDate::from_ymd_opt(1999, 4, 30).unwrap();
        let expected = chrono::NaiveDate::from_ymd_opt(1999, 5, 1).unwrap();

        assert_eq!(current, expected);

        let current = Day + chrono::NaiveDate::from_ymd_opt(1999, 2, 28).unwrap();
        let expected = chrono::NaiveDate::from_ymd_opt(1999, 3, 1).unwrap();

        assert_eq!(current, expected);

        let current = Day + chrono::NaiveDate::from_ymd_opt(2000, 2, 28).unwrap();
        let expected = chrono::NaiveDate::from_ymd_opt(2000, 2, 29).unwrap();

        assert_eq!(current, expected);

        let current = Day + chrono::NaiveDate::from_ymd_opt(2000, 2, 29).unwrap();
        let expected = chrono::NaiveDate::from_ymd_opt(2000, 3, 1).unwrap();

        assert_eq!(current, expected);
    }
}
