#[cfg(feature = "extended")]
mod extended;
mod list;
#[cfg(feature = "extended")]
mod note;
#[cfg(feature = "extended")]
mod period;
#[cfg(feature = "extended")]
mod recurrence;
mod simple;

#[cfg(feature = "extended")]
pub use extended::Extended;
pub use list::List;
#[cfg(feature = "extended")]
pub use note::Note;
#[cfg(feature = "extended")]
pub use period::Period;
#[cfg(feature = "extended")]
pub use recurrence::Recurrence;
pub use simple::Simple;

pub trait Task: From<String> + ToString + Clone + Default + AsRef<Simple> {}

impl Task for Simple {}

#[cfg(feature = "extended")]
impl Task for Extended {}

#[derive(Clone, Copy, Debug)]
pub(crate) enum Tag {
    Context,
    Project,
    Hashtag,
}

impl std::fmt::Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Context => "@",
            Self::Project => "+",
            Self::Hashtag => "#",
        };

        f.write_str(s)
    }
}
