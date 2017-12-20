use ::std::collections::BTreeMap;
use ::nom::rest_s;

named!(date<&str, ::Date>,
    do_parse!(
        year:
            take!(4) >>
            tag_s!("-") >>
        month:
            take!(2) >>
            tag_s!("-") >>
        day:
            take!(2) >>
            tag_s!(" ") >>
        ({
            let year = match year.parse() {
                Ok(year) => year,
                Err(_) => return ::nom::IResult::Error(::nom::ErrorKind::Custom(1)),
            };

            let month = match month.parse() {
                Ok(month) => month,
                Err(_) => return ::nom::IResult::Error(::nom::ErrorKind::Custom(2)),
            };

            let day = match day.parse() {
                Ok(day) => day,
                Err(_) => return ::nom::IResult::Error(::nom::ErrorKind::Custom(3)),
            };

            match ::Date::from_ymd_opt(year, month, day) {
                Some(date) => date,
                None => return ::nom::IResult::Error(::nom::ErrorKind::Custom(4)),
            }
        })
    )
);

named!(priority<&str, u8>,
    do_parse!(
            tag_s!("(") >>
        priority:
            take!(1) >>
            tag_s!(") ") >>
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
);

fn get_tags(delim: &str, subject: &str) -> Vec<String>
{
    let expression = format!("(?P<space>^|[\\s]){}(?P<tag>[\\w-]+)", delim);
    let regex = ::regex::Regex::new(&expression)
        .unwrap();

    let mut tags = regex.captures_iter(subject)
        .map(|x| {
            x["tag"].to_lowercase()
                .to_string()
        })
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>();

    tags.sort();
    tags.dedup();

    tags
}

fn get_contexts(subject: &str) -> Vec<String>
{
    get_tags("@", subject)
}

fn get_projects(subject: &str) -> Vec<String>
{
    get_tags("\\+", subject)
}

fn get_hashtags(subject: &str) -> Vec<String>
{
    get_tags("#", subject)
}

fn get_keywords(subject: &str) -> (String, BTreeMap<String, String>)
{
    let mut tags = BTreeMap::new();
    let regex = ::regex::Regex::new(r" (?P<key>[^\s]+):(?P<value>[^\s^/]+)").unwrap();

    let new_subject = regex.replace_all(subject, |caps: &::regex::Captures| {
        let key = caps.name("key").unwrap().as_str();
        let value = caps.name("value").unwrap().as_str();

        tags.insert(key.to_owned(), value.to_owned());

        String::new()
    });

    (new_subject.into_owned(), tags)
}

named!(parse<&str, ::Task>,
    do_parse!(
        finished:
            opt!(complete!(tag_s!("x "))) >>
        priority:
            opt!(complete!(priority)) >>
        finish_date:
            opt!(complete!(date)) >>
        create_date:
            opt!(complete!(date)) >>
        rest:
            rest_s >>
        ({
            let mut task = ::Task {
                priority: match priority {
                    Some(priority) => priority,
                    None => 26,
                },
                create_date: if create_date.is_none() {
                    finish_date
                } else {
                    create_date
                },
                finish_date: if create_date.is_none() {
                    None
                } else {
                    finish_date
                },
                finished: finished.is_some(),
                contexts: get_contexts(rest),
                projects: get_projects(rest),
                hashtags: get_hashtags(rest),

                .. Default::default()
            };

            let (subject, mut tags) = get_keywords(rest);
            task.subject = subject;

            if let Some(due) = tags.remove("due") {
                task.due_date = match ::Date::parse_from_str(due.as_str(), "%Y-%m-%d") {
                    Ok(due) => Some(due),
                    Err(_) => None,
                };
            }

            if let Some(t) = tags.remove("t") {
                task.threshold_date = match ::Date::parse_from_str(t.as_str(), "%Y-%m-%d") {
                    Ok(t) => Some(t),
                    Err(_) => None,
                };
            }

            task.tags = tags;

            task
        })
    )
);

pub fn task(line: &str) -> Result<::Task, ()>
{
    match parse(line) {
        ::nom::IResult::Done(_, task) => Ok(task),
        _ => Err(()),
    }
}
