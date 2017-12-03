#[derive(Debug, PartialEq)]
pub struct Task {
    pub subject: String,
    pub priority: u8,
    pub created: Option<::Date>,
    pub completed: Option<::Date>,
    pub finished: bool,
}
