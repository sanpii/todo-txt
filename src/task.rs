use ::std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
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
        Ok(
            ::parser::task(&s.to_owned())
        )
    }
}
