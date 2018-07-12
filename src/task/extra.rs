#[derive(Clone, Debug, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub struct Extra {
    #[cfg_attr(feature = "serde-support", serde(flatten))]
    pub inner: super::Task,
    #[cfg_attr(feature = "serde-support", serde(default))]
    pub note: super::Note,
    pub recurrence: Option<super::Recurrence>,
    #[cfg_attr(feature = "serde-support", serde(default))]
    pub flagged: bool,
}

impl Extra {
    pub fn complete(&mut self) {
        let today = ::chrono::Local::now().date().naive_local();

        self.finished = true;
        self.finish_date = Some(today);
    }

    pub fn uncomplete(&mut self) {
        self.finished = false;
        self.finish_date = None;
    }

    fn note(task: &super::Task) -> super::Note {
        if let Some(file) = task.tags.get(&Self::tag_name()) {
            super::Note::from_file(file)
        } else {
            super::Note::None
        }
    }

    fn tag_name() -> String {
        match ::std::env::var("TODO_NOTE_TAG") {
            Ok(tag) => tag,
            Err(_) => "note".to_owned(),
        }
    }
}

impl ::std::str::FromStr for Extra {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        let mut task = super::Task::from_str(s)?;

        let note = Self::note(&task);
        task.tags.remove(&Self::tag_name());

        let mut recurrence = None;

        if let Some(rec) = task.tags.get(&"rec".to_owned()) {
            recurrence = match super::Recurrence::from_str(rec) {
                Ok(rec) => Some(rec),
                Err(_) => None,
            };
        }
        task.tags.remove(&"rec".to_owned());

        let flagged = task.tags.contains_key(&"f".to_owned());
        task.tags.remove(&"f".to_owned());

        Ok(Self {
            note,
            inner: task,
            recurrence,
            flagged,
        })
    }
}

impl ::std::ops::Deref for Extra {
    type Target = super::Task;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl ::std::ops::DerefMut for Extra {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl ::std::fmt::Display for Extra {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use std::ops::Deref;

        f.write_str(format!("{}", self.deref()).as_str())?;

        if self.note != super::Note::None {
            f.write_str(format!(" {}", self.note).as_str())?;
        }

        if let Some(ref recurrence) = self.recurrence {
            f.write_str(format!(" rec:{}", recurrence).as_str())?;
        }

        if self.flagged {
            f.write_str(" f:1")?;
        }

        Ok(())
    }
}

impl ::std::cmp::PartialOrd for Extra {
    fn partial_cmp(&self, other: &Self) -> Option<::std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl ::std::cmp::Ord for Extra {
    fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
        if self.inner.due_date != other.inner.due_date {
            return self.inner.due_date.cmp(&other.inner.due_date);
        }

        if self.inner.priority != other.inner.priority {
            return self.inner.priority.cmp(&other.inner.priority).reverse();
        }

        if self.inner.subject != other.inner.subject {
            return self.inner.subject.cmp(&other.inner.subject);
        }

        ::std::cmp::Ordering::Equal
    }
}
