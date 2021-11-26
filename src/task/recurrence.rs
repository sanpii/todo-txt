#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde-support",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub struct Recurrence {
    pub num: i64,
    pub period: super::Period,
    #[cfg_attr(feature = "serde-support", serde(default))]
    pub strict: bool,
}

impl std::str::FromStr for Recurrence {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = String::from(s);

        let strict = if s.get(0..1) == Some("+") {
            s.remove(0);
            true
        } else {
            false
        };

        let period = match s.pop() {
            Some(c) => super::Period::from_str(&c.to_string())?,
            None => return Err(crate::Error::InvalidRecurrence(s.to_string())),
        };

        let num = s
            .parse()
            .map_err(|_| crate::Error::InvalidRecurrence(s.to_string()))?;

        Ok(Self {
            num,
            period,
            strict,
        })
    }
}

impl std::fmt::Display for Recurrence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.strict {
            f.write_str("+")?;
        }

        f.write_str(format!("{}{}", self.num, self.period).as_str())?;

        Ok(())
    }
}

impl std::ops::Add<chrono::NaiveDate> for Recurrence {
    type Output = chrono::NaiveDate;

    fn add(self, rhs: Self::Output) -> Self::Output {
        use super::Period::{self, *};
        use chrono::{Datelike, Duration};

        let delta_months = match self.period {
            #[allow(clippy::suspicious_arithmetic_impl)]
            Year => 12 * self.num as u32,
            Month => self.num as u32,
            Week => return rhs + Duration::weeks(self.num),
            Day => return rhs + Duration::days(self.num),
        };

        let mut y = rhs.year();
        let mut m = rhs.month();
        let mut d = rhs.day();

        // Semantics taken from
        //  https://github.com/dbeniamine/todo.txt-vim/blob/259125d9efe93f69582f50ef68c17e20fd1e963a/autoload/todo.vim#L531-L538
        let was_last_day = d == Period::days_in_month(m, y);

        m += delta_months;
        y += ((m - 1) / 12) as i32;
        m = (m - 1) % 12 + 1;
        if was_last_day || d > Period::days_in_month(m, y) {
            d = Period::days_in_month(m, y);
        }

        chrono::NaiveDate::from_ymd(y, m, d)
    }
}
