extern crate todo_txt;

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;

    #[test]
    fn simple_task() {
        let line = "Email SoAndSo at soandso@example.com".to_owned();
        let task = todo_txt::Task {
            subject: "Email SoAndSo at soandso@example.com".to_owned(),
            ..Default::default()
        };

        assert_eq!(todo_txt::parser::task(&line), Ok(task));
    }

    #[test]
    fn finished() {
        let line = "x done".to_owned();
        let task = todo_txt::Task {
            subject: "done".to_owned(),
            finished: true,
            ..Default::default()
        };

        assert_eq!(todo_txt::parser::task(&line), Ok(task));
    }

    #[test]
    fn created() {
        let line = "x 2017-11-25 subject".to_owned();
        let task = todo_txt::Task {
            subject: "subject".to_owned(),
            create_date: Some(todo_txt::Date::from_ymd(2017, 11, 25)),
            finished: true,
            ..Default::default()
        };

        assert_eq!(todo_txt::parser::task(&line), Ok(task));
    }

    #[test]
    fn invalid_date() {
        let line = "2017-02-30 subject".to_owned();
        let task = todo_txt::Task {
            subject: "2017-02-30 subject".to_owned(),

            ..Default::default()
        };

        assert_eq!(todo_txt::parser::task(&line), Ok(task));
    }

    #[test]
    fn completed() {
        let line = "x 2017-11-26 2017-11-25 subject".to_owned();
        let task = todo_txt::Task {
            subject: "subject".to_owned(),
            create_date: Some(todo_txt::Date::from_ymd(2017, 11, 25)),
            finish_date: Some(todo_txt::Date::from_ymd(2017, 11, 26)),
            finished: true,
            ..Default::default()
        };

        assert_eq!(todo_txt::parser::task(&line), Ok(task));
    }

    #[test]
    fn priority() {
        let line = "x (A) 2017-11-26 2017-11-25 subject".to_owned();
        let task = todo_txt::Task {
            subject: "subject".to_owned(),
            priority: 0,
            create_date: Some(todo_txt::Date::from_ymd(2017, 11, 25)),
            finish_date: Some(todo_txt::Date::from_ymd(2017, 11, 26)),
            finished: true,
            ..Default::default()
        };

        assert_eq!(todo_txt::parser::task(&line), Ok(task));
    }

    #[test]
    fn contexts() {
        let line = "Email SoAndSo at soandso@example.com @context1 @context2".to_owned();
        let task = todo_txt::Task {
            subject: "Email SoAndSo at soandso@example.com @context1 @context2".to_owned(),
            contexts: vec!["context1".to_owned(), "context2".to_owned()],
            ..Default::default()
        };

        assert_eq!(todo_txt::parser::task(&line), Ok(task));
    }

    #[test]
    fn deplucate_contexts() {
        let line = "Email SoAndSo at soandso@example.com @context1 @context2 @context1".to_owned();
        let task = todo_txt::Task {
            subject: "Email SoAndSo at soandso@example.com @context1 @context2 @context1"
                .to_owned(),
            contexts: vec!["context1".to_owned(), "context2".to_owned()],
            ..Default::default()
        };

        assert_eq!(todo_txt::parser::task(&line), Ok(task));
    }

    #[test]
    fn projects() {
        let line = "Email SoAndSo at soandso@example.com +project1 +project1\\subject1 @context2"
            .to_owned();
        let task = todo_txt::Task {
            subject: "Email SoAndSo at soandso@example.com +project1 +project1\\subject1 @context2"
                .to_owned(),
            contexts: vec!["context2".to_owned()],
            projects: vec!["project1".to_owned(), "project1\\subject1".to_owned()],
            ..Default::default()
        };

        assert_eq!(todo_txt::parser::task(&line), Ok(task));
    }

    #[test]
    fn empty_tag() {
        let line = "Email SoAndSo at soandso@example.com + @ #".to_owned();
        let task = todo_txt::Task {
            subject: "Email SoAndSo at soandso@example.com + @ #".to_owned(),

            ..Default::default()
        };

        assert_eq!(todo_txt::parser::task(&line), Ok(task));
    }

    #[test]
    fn case_insensitive_tag() {
        let line = "Email SoAndSo at soandso@example.com +Project1".to_owned();
        let task = todo_txt::Task {
            subject: "Email SoAndSo at soandso@example.com +Project1".to_owned(),
            projects: vec!["project1".to_owned()],

            ..Default::default()
        };

        assert_eq!(todo_txt::parser::task(&line), Ok(task));
    }

    #[test]
    fn start_with_tag() {
        let line = "+Project1".to_owned();
        let task = todo_txt::Task {
            subject: "+Project1".to_owned(),
            projects: vec!["project1".to_owned()],

            ..Default::default()
        };

        assert_eq!(todo_txt::parser::task(&line), Ok(task));
    }

    #[test]
    fn url() {
        let line = "Participer à https://contributopia.org".to_owned();
        let task = todo_txt::Task {
            subject: line.clone(),

            ..Default::default()
        };

        assert_eq!(todo_txt::parser::task(&line), Ok(task));
    }

    #[test]
    fn hashtags() {
        let line = "Email SoAndSo at soandso@example.com +project1 #tag @context2".to_owned();
        let task = todo_txt::Task {
            subject: "Email SoAndSo at soandso@example.com +project1 #tag @context2".to_owned(),
            contexts: vec!["context2".to_owned()],
            projects: vec!["project1".to_owned()],
            hashtags: vec!["tag".to_owned()],
            ..Default::default()
        };

        assert_eq!(todo_txt::parser::task(&line), Ok(task));
    }

    #[test]
    fn keywords() {
        let mut keywords = BTreeMap::new();
        keywords.insert("key1".to_owned(), "2018-01-01".to_owned());
        keywords.insert("key2".to_owned(), "value".to_owned());

        let line = "Email SoAndSo at soandso@example.com key1:2018-01-01 key2:value".to_owned();
        let task = todo_txt::Task {
            subject: "Email SoAndSo at soandso@example.com".to_owned(),
            tags: keywords,
            ..Default::default()
        };

        assert_eq!(todo_txt::parser::task(&line), Ok(task));
    }

    #[test]
    fn due() {
        let line = "Email SoAndSo at soandso@example.com due:2018-01-01".to_owned();
        let task = todo_txt::Task {
            subject: "Email SoAndSo at soandso@example.com".to_owned(),
            due_date: Some(todo_txt::Date::from_ymd(2018, 1, 1)),

            ..Default::default()
        };

        assert_eq!(todo_txt::parser::task(&line), Ok(task));
    }

    #[test]
    fn threshold() {
        let line = "Email SoAndSo at soandso@example.com t:2018-01-01".to_owned();
        let task = todo_txt::Task {
            subject: "Email SoAndSo at soandso@example.com".to_owned(),
            threshold_date: Some(todo_txt::Date::from_ymd(2018, 1, 1)),

            ..Default::default()
        };

        assert_eq!(todo_txt::parser::task(&line), Ok(task));
    }

    #[test]
    fn begin_with_keyword() {
        let line = "(C) t:2018-04-03 Open issue on todo-txt parser".to_owned();

        let task = todo_txt::Task {
            subject: "Open issue on todo-txt parser".to_owned(),
            threshold_date: Some(todo_txt::Date::from_ymd(2018, 4, 3)),
            priority: 2,

            ..Default::default()
        };

        assert_eq!(todo_txt::parser::task(&line), Ok(task));
    }

    #[test]
    fn url_in_tags() {
        let mut keywords = BTreeMap::new();
        keywords.insert("url".to_owned(), "http://example.org".to_owned());

        let line = "2018-03-26 test url:http://example.org".to_owned();
        let task = todo_txt::Task {
            subject: "test".to_owned(),
            create_date: Some(todo_txt::Date::from_ymd(2018, 3, 26)),
            tags: keywords,

            ..Default::default()
        };

        assert_eq!(todo_txt::parser::task(&line), Ok(task));
    }
}
