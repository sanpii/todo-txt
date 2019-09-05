#[cfg(feature = "serde-support")]
#[macro_use]
extern crate serde_derive;

pub mod parser;
pub mod task;

pub use crate::task::Task;
pub use chrono::NaiveDate as Date;
