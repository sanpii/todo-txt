#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate chrono;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate nom;
extern crate regex;

#[cfg(feature = "serde-support")]
extern crate serde;

#[cfg(feature = "serde-support")]
#[macro_use]
extern crate serde_derive;

pub mod parser;
pub mod task;

pub use chrono::NaiveDate as Date;
pub use task::Task;
