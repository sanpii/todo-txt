#[cfg(feature = "serde-support")]
extern crate serde_json;
extern crate todo_txt;

use std::collections::BTreeMap;

#[test]
#[cfg(all(feature = "serde-support", feature="extended"))]
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
            tags: BTreeMap::new(),
        },
        flagged: false,
        note: ::todo_txt::task::Note::None,
        recurrence: None,
    };

    assert_eq!(actual, expected);
}

#[test]
#[cfg(all(feature = "serde-support", feature="extended"))]
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

#[test]
#[cfg(feature="extended")]
fn test_from_task() {
    let task = ::todo_txt::Task {
        subject: "Subject".to_string(),
        tags: {
            let mut map = BTreeMap::new();
            map.insert("f".to_string(), "1".to_string());

            map
        },

        ..::todo_txt::Task::default()
    };

    let extra: ::todo_txt::task::Extra = task.into();

    assert_eq!(extra.flagged, true);
}
