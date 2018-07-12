extern crate todo_txt;

#[test]
fn from_invalid() {
    use std::str::FromStr;

    assert_eq!(::todo_txt::task::Recurrence::from_str("1"), Err(()));
}
