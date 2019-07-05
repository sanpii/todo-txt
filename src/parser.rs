use nom::combinator::rest;
use std::collections::BTreeMap;

macro_rules! return_error (
    ($num:expr) => {
        return Err(
            nom::Err::Error(
                nom::error::make_error(
                    "",
                    nom::error::ErrorKind::Tag
                )
            )
        );
    }
);

fn date(input: &str) -> nom::IResult<&str, crate::Date>
{
    do_parse!(input,
        year:
            take!(4) >>
            tag!("-") >>
        month:
            take!(2) >>
            tag!("-") >>
        day:
            take!(2) >>
            tag!(" ") >>
        ({
            let year = match year.parse() {
                Ok(year) => year,
                Err(_) => return_error!(1),
            };

            let month = match month.parse() {
                Ok(month) => month,
                Err(_) => return_error!(2),
            };

            let day = match day.parse() {
                Ok(day) => day,
                Err(_) => return_error!(3),
            };

            match crate::Date::from_ymd_opt(year, month, day) {
                Some(date) => date,
                None => return_error!(4),
            }
        })
    )
}

fn priority(input: &str) -> nom::IResult <&str, u8>
{
    do_parse!(input,
            tag!("(") >>
        priority:
            take!(1) >>
            tag!(") ") >>
        ({
            let p = priority.as_bytes()[0];

            if p >= b'A' && p <= b'Z' {
                p - b'A'
            }
            else {
                26
            }
        })
    )
}

fn get_tags(regex: &::regex::Regex, subject: &str) -> Vec<String> {
    let mut tags = regex
        .captures_iter(subject)
        .map(|x| x["tag"].to_lowercase().to_string())
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>();

    tags.sort();
    tags.dedup();

    tags
}

macro_rules! regex_tags_shared {
    () => {
        "(?P<space>^|[\\s]){}(?P<tag>[\\w\\\\-]+)"
    };
}

fn get_contexts(subject: &str) -> Vec<String> {
    lazy_static! {
        static ref REGEX: regex::Regex =
            regex::Regex::new(&format!(regex_tags_shared!(), "@")).unwrap();
    }
    get_tags(&REGEX, subject)
}

fn get_projects(subject: &str) -> Vec<String> {
    lazy_static! {
        static ref REGEX: regex::Regex =
            regex::Regex::new(&format!(regex_tags_shared!(), "\\+")).unwrap();
    }
    get_tags(&REGEX, subject)
}

fn get_hashtags(subject: &str) -> Vec<String> {
    lazy_static! {
        static ref REGEX: regex::Regex =
            regex::Regex::new(&format!(regex_tags_shared!(), "#")).unwrap();
    }
    get_tags(&REGEX, subject)
}

fn get_keywords(subject: &str) -> (String, BTreeMap<String, String>) {
    lazy_static! {
        static ref REGEX: regex::Regex =
            regex::Regex::new(r"(\s+|^)(?P<key>[^\s]+?):(?P<value>[^\s]+)").unwrap();
    }

    let mut tags = BTreeMap::new();

    let new_subject = REGEX.replace_all(subject, |caps: &::regex::Captures| {
        let key = caps.name("key").unwrap().as_str();
        let value = caps.name("value").unwrap().as_str();

        if value.starts_with('/') {
            format!(" {}:{}", key, value)
        } else {
            tags.insert(key.to_owned(), value.to_owned());

            String::new()
        }
    });

    (new_subject.trim().to_owned(), tags)
}

fn parse(input: &str) -> nom::IResult<&str, crate::Task>
{
    do_parse!(input,
        finished:
            opt!(complete!(tag!("x "))) >>
        priority:
            opt!(complete!(priority)) >>
        finish_date:
            opt!(complete!(date)) >>
        create_date:
            opt!(complete!(date)) >>
        rest:
            rest >>
        ({
            let mut task = crate::Task {
                priority: priority.unwrap_or(26),
                create_date: create_date.or(finish_date),
                finish_date: create_date.and(finish_date),
                finished: finished.is_some(),
                contexts: get_contexts(rest),
                projects: get_projects(rest),
                hashtags: get_hashtags(rest),

                .. crate::Task::default()
            };

            let (subject, mut tags) = get_keywords(rest);
            task.subject = subject;

            if let Some(due) = tags.remove("due") {
                task.due_date = match crate::Date::parse_from_str(due.as_str(), "%Y-%m-%d") {
                    Ok(due) => Some(due),
                    Err(_) => None,
                };
            }

            if let Some(t) = tags.remove("t") {
                task.threshold_date = match crate::Date::parse_from_str(t.as_str(), "%Y-%m-%d") {
                    Ok(t) => Some(t),
                    Err(_) => None,
                };
            }

            task.tags = tags;

            task
        })
    )
}

pub fn task(line: &str) -> Result<crate::Task, ()> {
    match parse(line) {
        Ok((_, task)) => Ok(task),
        _ => Err(()),
    }
}
