#[cfg(feature = "serde-support")]
extern crate serde_json;
extern crate todo_txt;

#[test]
fn from_str() {
    use std::str::FromStr;

    let line = "Email SoAndSo at soandso@example.com";
    let task = ::todo_txt::Task {
        subject: "Email SoAndSo at soandso@example.com".to_owned(),
        ..Default::default()
    };

    assert_eq!(::todo_txt::Task::from_str(line), Ok(task));
}

#[test]
fn display() {
    let task = ::todo_txt::Task {
        subject: "@Email SoAndSo at soandso@example.com".to_owned(),
        priority: 1,
        finished: true,
        due_date: Some(::todo_txt::Date::from_ymd(2019, 2, 10)),
        finish_date: Some(::todo_txt::Date::from_ymd(2019, 2, 15)),
        create_date: Some(::todo_txt::Date::from_ymd(2019, 2, 5)),

        ..Default::default()
    };

    let line = format!("{}", task);

    assert_eq!(
        line,
        "x (B) 2019-02-15 2019-02-05 @Email SoAndSo at soandso@example.com due:2019-02-10"
    );
}

#[test]
#[cfg(feature = "serde-support")]
fn test_simple_deserialize() {
    let json = r#"{
        "subject": "Test",
        "priority": 26,
        "create_date": null,
        "finish_date": null,
        "finished": false,
        "contexts": [ "context_a", "context_b" ],
        "threshold_date": null,
        "due_date": null,
        "projects": [],
        "hashtags": [ "tag_a", "tag_b" ],
        "tags": {}
    }"#;

    let task: ::todo_txt::Task = ::serde_json::from_str(json).unwrap();

    assert_eq!(task.subject, "Test");
    assert_eq!(task.priority, 26);
    assert_eq!(task.create_date, None);
    assert_eq!(task.finish_date, None);
    assert_eq!(task.contexts[0], "context_a");
    assert_eq!(task.contexts[1], "context_b");
    assert_eq!(task.threshold_date, None);
    assert_eq!(task.due_date, None);
    assert_eq!(task.hashtags[0], "tag_a");
    assert_eq!(task.hashtags[1], "tag_b");

    assert!(!task.finished);
    assert!(task.projects.is_empty());
    assert!(task.tags.is_empty());
}

#[test]
#[cfg(feature = "serde-support")]
fn test_deserialize_with_dates() {
    let json = r#"{
        "subject": "Test",
        "priority": 26,
        "create_date": "2018-03-01",
        "finish_date": "2018-03-04",
        "finished": false,
        "contexts": [ "context_a", "context_b" ],
        "threshold_date": "2018-03-02",
        "due_date": "2018-03-03",
        "projects": [],
        "hashtags": [ "tag_a", "tag_b" ],
        "tags": {}
    }"#;

    let task: ::todo_txt::Task = ::serde_json::from_str(json).unwrap();

    assert_eq!(task.subject, "Test");
    assert_eq!(task.priority, 26);
    assert_eq!(
        task.create_date,
        Some(::todo_txt::Date::from_ymd(2018, 03, 01))
    );
    assert_eq!(
        task.finish_date,
        Some(::todo_txt::Date::from_ymd(2018, 03, 04))
    );
    assert!(!task.finished);
    assert_eq!(task.contexts[0], "context_a");
    assert_eq!(task.contexts[1], "context_b");
    assert_eq!(
        task.threshold_date,
        Some(::todo_txt::Date::from_ymd(2018, 03, 02))
    );
    assert_eq!(
        task.due_date,
        Some(::todo_txt::Date::from_ymd(2018, 03, 03))
    );
    assert!(task.projects.is_empty());
    assert_eq!(task.hashtags[0], "tag_a");
    assert_eq!(task.hashtags[1], "tag_b");
    assert!(task.tags.is_empty());
}

#[test]
#[cfg(feature = "serde-support")]
fn test_serialize_simple() {
    let task = ::todo_txt::Task::default();
    let json = ::serde_json::to_string(&task).unwrap();

    let expected = r#"{"subject":"","priority":26,"create_date":null,"finish_date":null,"finished":false,"threshold_date":null,"due_date":null,"contexts":[],"projects":[],"hashtags":[],"tags":{}}"#;

    assert_eq!(expected, json);
}
