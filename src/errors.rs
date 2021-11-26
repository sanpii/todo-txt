pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Launch this program via todo.sh")]
    Env,
    #[error("Invalid period: {0}")]
    InvalidPeriod(String),
    #[error("Invalid priority: {0}")]
    InvalidPriority(char),
    #[error("Invalid recurrence: {0}")]
    InvalidRecurrence(String),
    #[error("Unable to save note: {0}")]
    Note(std::io::Error),
}
