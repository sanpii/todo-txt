extern crate todo_txt;

#[test]
fn from_str()
{
    use ::std::str::FromStr;

    let line = "Email SoAndSo at soandso@example.com\n";
    let task = ::todo_txt::Task {
        subject: "Email SoAndSo at soandso@example.com".to_owned(),
        .. Default::default()
    };

    assert_eq!(::todo_txt::Task::from_str(line), Ok(task));
}
