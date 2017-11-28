extern crate todo_txt;

#[test]
fn simple_task()
{
    let line = "Email SoAndSo at soandso@example.com\n".to_owned();
    let task = ::todo_txt::Task {
        subject: "Email SoAndSo at soandso@example.com".to_owned(),
        created: None,
        finished: false,
    };

    assert_eq!(::todo_txt::parser::task(&line), task);
}

#[test]
fn finished_task()
{
    let line = "x done\n".to_owned();
    let task = ::todo_txt::Task {
        subject: "done".to_owned(),
        created: None,
        finished: true,
    };

    assert_eq!(::todo_txt::parser::task(&line), task);
}

#[test]
fn created_task()
{
    let line = "x 2017-11-25 done\n".to_owned();
    let task = ::todo_txt::Task {
        subject: "done".to_owned(),
        created: Some(::todo_txt::Date::from_ymd(2017, 11, 25)),
        finished: true,
    };

    assert_eq!(::todo_txt::parser::task(&line), task);
}
