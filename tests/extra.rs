#[cfg(feature = "serde-support")]
extern crate serde_json;
extern crate todo_txt;

#[test]
#[cfg(feature = "serde-support")]
fn test_extra_deserialize() {
    let json = r#"{
        "subject": "Test"
    }"#;

    let actual: ::todo_txt::task::Extra = ::serde_json::from_str(json).unwrap();

    let expected = ::todo_txt::task::Extra {
        inner: ::todo_txt::Task {
            subject: "Test".to_string(),
            priority: 26,
            create_date: None,
            finish_date: None,
            contexts: Vec::new(),
            threshold_date: None,
            due_date: None,
            hashtags: Vec::new(),
            finished: false,
            projects: Vec::new(),
            tags: ::std::collections::BTreeMap::new(),
        },
        flagged: false,
        note: ::todo_txt::task::Note::None,
        recurrence: None,
    };

    assert_eq!(actual, expected);
}

#[test]
#[cfg(feature = "serde-support")]
fn test_note_deserialize() {
    let json = r#"{
        "subject": "Test",
        "note": "A note"
    }"#;

    let actual: ::todo_txt::task::Extra = ::serde_json::from_str(json).unwrap();

    let expected = ::todo_txt::task::Extra {
        inner: ::todo_txt::Task {
            subject: "Test".to_string(),

            ..::todo_txt::Task::default()
        },
        note: ::todo_txt::task::Note::Short("A note".to_string()),

        ..::todo_txt::task::Extra::default()
    };

    assert_eq!(actual, expected);
}
