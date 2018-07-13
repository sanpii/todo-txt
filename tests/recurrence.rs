extern crate todo_txt;

#[cfg(all(test, feature = "extended"))]
mod test {
    #[test]
    fn from_invalid() {
        use std::str::FromStr;

        assert_eq!(::todo_txt::task::Recurrence::from_str("1"), Err(()));
    }
}
