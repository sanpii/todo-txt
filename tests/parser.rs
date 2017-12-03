extern crate todo_txt;

#[test]
fn simple_task()
{
    let line = "Email SoAndSo at soandso@example.com\n".to_owned();
    let task = ::todo_txt::Task {
        subject: "Email SoAndSo at soandso@example.com".to_owned(),
        created: None,
        completed: None,
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
        completed: None,
        finished: true,
    };

    assert_eq!(::todo_txt::parser::task(&line), task);
}

#[test]
fn created_task()
{
    let line = "x 2017-11-25 subject\n".to_owned();
    let task = ::todo_txt::Task {
        subject: "subject".to_owned(),
        created: Some(::todo_txt::Date::from_ymd(2017, 11, 25)),
        completed: None,
        finished: true,
    };

    assert_eq!(::todo_txt::parser::task(&line), task);
}


#[test]
fn completed_task()
{
    let line = "x 2017-11-26 2017-11-25 subject\n".to_owned();
    let task = ::todo_txt::Task {
        subject: "subject".to_owned(),
        created: Some(::todo_txt::Date::from_ymd(2017, 11, 25)),
        completed: Some(::todo_txt::Date::from_ymd(2017, 11, 26)),
        finished: true,
    };

    assert_eq!(::todo_txt::parser::task(&line), task);
}
