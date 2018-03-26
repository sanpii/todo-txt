use ::std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(any(feature = "serde-support", test), derive(Serialize, Deserialize))]
pub struct Task {
    pub subject: String,
    pub priority: u8,
    pub create_date: Option<::Date>,
    pub finish_date: Option<::Date>,
    pub finished: bool,
    pub threshold_date: Option<::Date>,
    pub due_date: Option<::Date>,
    pub contexts: Vec<String>,
    pub projects: Vec<String>,
    pub hashtags: Vec<String>,
    pub tags: BTreeMap<String, String>,
}

impl Default for Task
{
    fn default() -> Self
    {
        Self {
            subject: String::new(),
            priority: 26,
            create_date: None,
            finish_date: None,
            finished: false,
            threshold_date: None,
            due_date: None,
            contexts: Vec::new(),
            projects: Vec::new(),
            hashtags: Vec::new(),
            tags: BTreeMap::new(),
        }
    }
}

impl ::std::str::FromStr for Task
{
    type Err = ();

    fn from_str(s: &str) -> Result<Task, ()>
    {
        ::parser::task(&s.to_owned())
    }
}

impl ::std::fmt::Display for Task
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result
    {
        if self.finished {
            f.write_str("x ")?;
        }

        if self.priority < 26 {
            let priority = (b'A' + self.priority) as char;

            f.write_str(format!("({}) ", priority).as_str())?;
        }

        if let Some(finish_date) = self.finish_date {
            f.write_str(format!("{} ", finish_date.format("%Y-%m-%d")).as_str())?;
        }

        if let Some(create_date) = self.create_date {
            f.write_str(format!("{} ", create_date.format("%Y-%m-%d")).as_str())?;
        }

        f.write_str(self.subject.as_str())?;

        if let Some(due_date) = self.due_date {
            f.write_str(format!(" due:{}", due_date.format("%Y-%m-%d")).as_str())?;
        }

        if let Some(threshold_date) = self.threshold_date {
            f.write_str(format!(" t:{}", threshold_date.format("%Y-%m-%d")).as_str())?;
        }

        for (key, value) in &self.tags {
            f.write_str(format!(" {}:{}", key, value).as_str())?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Task;

    #[test]
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

        let task : Task = ::serde_json::from_str(json).unwrap();

        assert_eq!(task.subject, "Test");
        assert_eq!(task.priority, 26);
        assert_eq!(task.create_date, None);
        assert_eq!(task.finish_date, None);
        assert!(!task.finished);
        assert_eq!(task.contexts[0], "context_a");
        assert_eq!(task.contexts[1], "context_b");
        assert_eq!(task.threshold_date, None);
        assert_eq!(task.due_date, None);
        assert!(task.projects.is_empty());
        assert_eq!(task.hashtags[0], "tag_a");
        assert_eq!(task.hashtags[1], "tag_b");
        assert!(task.tags.is_empty());
    }

    #[test]
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

        let task : Task = ::serde_json::from_str(json).unwrap();

        assert_eq!(task.subject, "Test");
        assert_eq!(task.priority, 26);
        assert_eq!(task.create_date, Some(::Date::from_ymd(2018, 03, 01)));
        assert_eq!(task.finish_date, Some(::Date::from_ymd(2018, 03, 04)));
        assert!(!task.finished);
        assert_eq!(task.contexts[0], "context_a");
        assert_eq!(task.contexts[1], "context_b");
        assert_eq!(task.threshold_date, Some(::Date::from_ymd(2018, 03, 02)));
        assert_eq!(task.due_date, Some(::Date::from_ymd(2018, 03, 03)));
        assert!(task.projects.is_empty());
        assert_eq!(task.hashtags[0], "tag_a");
        assert_eq!(task.hashtags[1], "tag_b");
        assert!(task.tags.is_empty());
    }

    #[test]
    fn test_serialize_simple() {
        let task = Task::default();
        let json = ::serde_json::to_string(&task).unwrap();

        let expected = r#"{"subject":"","priority":26,"create_date":null,"finish_date":null,"finished":false,"threshold_date":null,"due_date":null,"contexts":[],"projects":[],"hashtags":[],"tags":{}}"#;

        assert_eq!(expected, json);
    }

}
