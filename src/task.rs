use ::std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq)]
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

        for (key, value) in self.tags.iter() {
            f.write_str(format!(" {}:{}", key, value).as_str())?;
        }

        Ok(())
    }
}
