extern crate todo_txt;

#[test]
fn simple_task()
{
    let line = "Email SoAndSo at soandso@example.com\n".to_owned();
    let task = ::todo_txt::Task {
        subject: "Email SoAndSo at soandso@example.com".to_owned(),
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
        finished: true,
    };

    assert_eq!(::todo_txt::parser::task(&line), task);
}
