extern crate todo_txt;

#[test]
fn from_str()
{
    use ::std::str::FromStr;

    let line = "Email SoAndSo at soandso@example.com";
    let task = ::todo_txt::Task {
        subject: "Email SoAndSo at soandso@example.com".to_owned(),
        .. Default::default()
    };

    assert_eq!(::todo_txt::Task::from_str(line), Ok(task));
}

#[test]
fn display()
{
    let task = ::todo_txt::Task {
        subject: "@Email SoAndSo at soandso@example.com".to_owned(),
        priority: 1,
        finished: true,
        due_date: Some(::todo_txt::Date::from_ymd(2019, 2, 10)),
        finish_date: Some(::todo_txt::Date::from_ymd(2019, 2, 15)),
        create_date: Some(::todo_txt::Date::from_ymd(2019, 2, 5)),

        .. Default::default()
    };

    let line = format!("{}", task);

    assert_eq!(line, "x (B) 2019-02-15 2019-02-05 @Email SoAndSo at soandso@example.com due:2019-02-10");
}
