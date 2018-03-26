#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate chrono;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate nom;
extern crate regex;

#[cfg(any(feature = "serde-support", test))]
extern crate serde;

#[cfg(any(feature = "serde-support", test))]
#[macro_use]
extern crate serde_derive;

#[cfg(test)]
extern crate serde_json;

pub mod parser;
pub mod task;

pub use chrono::NaiveDate as Date;
pub use task::Task;
