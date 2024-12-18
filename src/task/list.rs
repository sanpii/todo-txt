#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct List<T: crate::Task> {
    pub tasks: Vec<T>,
}

impl<T: crate::Task> List<T> {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn from(text: &str) -> Self {
        let mut list = Self::new();

        for line in text.split('\n') {
            if line.is_empty() {
                continue;
            }

            list.tasks.push(line.to_string().into());
        }

        list
    }

    #[must_use]
    pub fn get(&self, index: &usize) -> &T {
        &self.tasks[*index - 1]
    }

    pub fn get_mut(&mut self, index: &usize) -> &mut T {
        &mut self.tasks[*index - 1]
    }

    pub fn remove(&mut self, index: usize) -> T {
        self.tasks.remove(index - 1)
    }

    #[must_use]
    pub fn projects(&self) -> Vec<String> {
        let mut projects = self.tasks.iter().fold(Vec::new(), |mut acc, item| {
            acc.extend_from_slice(&item.as_ref().projects());

            acc
        });
        projects.sort();
        projects.dedup();

        projects
    }

    #[must_use]
    pub fn contexts(&self) -> Vec<String> {
        let mut contexts = self.tasks.iter().fold(Vec::new(), |mut acc, item| {
            acc.extend_from_slice(&item.as_ref().contexts());

            acc
        });
        contexts.sort();
        contexts.dedup();

        contexts
    }
}

impl<T: crate::Task> From<String> for List<T> {
    fn from(value: String) -> Self {
        Self::from(&value)
    }
}

impl<T: crate::Task> std::str::FromStr for List<T> {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}

impl<T: crate::Task> std::fmt::Display for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for task in &self.tasks {
            writeln!(f, "{}", task.to_string())?;
        }

        Ok(())
    }
}

impl<T: crate::Task> std::ops::Deref for List<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.tasks
    }
}

impl<T: crate::Task> std::ops::DerefMut for List<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tasks
    }
}

impl<T: crate::Task> FromIterator<T> for List<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            tasks: iter.into_iter().collect(),
        }
    }
}

impl<'a, T: crate::Task> FromIterator<&'a T> for List<T> {
    fn from_iter<I: IntoIterator<Item = &'a T>>(iter: I) -> Self {
        Self {
            tasks: iter.into_iter().cloned().collect(),
        }
    }
}
