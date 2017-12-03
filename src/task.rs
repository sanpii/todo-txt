#[derive(Debug, PartialEq)]
pub struct Task {
    pub subject: String,
    pub priority: u8,
    pub create_date: Option<::Date>,
    pub finish_date: Option<::Date>,
    pub finished: bool,
    pub contexts: Vec<String>,
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
            contexts: Vec::new(),
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
