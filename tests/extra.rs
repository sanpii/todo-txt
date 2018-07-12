#[cfg(feature = "serde-support")]
extern crate serde_json;
extern crate todo_txt;

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

    let actual: ::todo_txt::task::Extra = ::serde_json::from_str(json).unwrap();

    let expected = ::todo_txt::task::Extra {
        inner: ::todo_txt::Task {
            subject: "Test".to_string(),
            priority: 26,
            create_date: None,
            finish_date: None,
            contexts: vec!["context_a".to_string(), "context_b".to_string()],
            threshold_date: None,
            due_date: None,
            hashtags: vec!["tag_a".to_string(), "tag_b".to_string()],
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
