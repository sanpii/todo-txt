#[derive(Debug, PartialEq)]
pub struct Task {
    pub subject: String,
    pub created: Option<::Date>,
    pub completed: Option<::Date>,
    pub finished: bool,
}
