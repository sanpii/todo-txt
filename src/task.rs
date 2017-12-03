#[derive(Debug, PartialEq)]
pub struct Task {
    pub subject: String,
    pub priority: u8,
    pub create_date: Option<::Date>,
    pub finish_date: Option<::Date>,
    pub finished: bool,
}
