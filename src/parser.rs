fn is_space(chr: char) -> bool
{
    chr == ' ' || chr == '\t' || is_line_ending(chr)
}

fn is_line_ending(chr: char) -> bool
{
    chr == '\n' || chr == '\r'
}

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
        (
            ::Date::from_ymd(
                year.parse().unwrap(),
                month.parse().unwrap(),
                day.parse().unwrap()
            )
        )
    )
);

named!(priority<&str, u8>,
    do_parse!(
            tag_s!("(") >>
        priority:
            take!(1) >>
            tag_s!(") ") >>
        (
            priority.as_bytes()[0] - b'A'
        )
    )
);

named!(context<&str, String>,
    do_parse!(
            take_until_and_consume_s!(" @") >>
        context:
            take_till!(is_space) >>
        (
            context.to_owned()
        )
    )
);

named!(contexts<&str, Vec<String>>, many0!(context));

fn get_contexts(subject: &str) -> Vec<String>
{
    let mut contexts = contexts(subject).unwrap().1;

    contexts.sort();
    contexts.dedup();

    contexts
}

named!(project<&str, String>,
    do_parse!(
            take_until_and_consume_s!(" +") >>
        project:
            take_till!(is_space) >>
        (
            project.to_owned()
        )
    )
);

named!(projects<&str, Vec<String>>, many0!(project));

fn get_projects(subject: &str) -> Vec<String>
{
    let mut projects = projects(subject).unwrap().1;

    projects.sort();
    projects.dedup();

    projects
}

named!(hashtag<&str, String>,
    do_parse!(
            take_until_and_consume_s!(" #") >>
        hashtag:
            take_till!(is_space) >>
        (
            hashtag.to_owned()
        )
    )
);

named!(hashtags<&str, Vec<String>>, many0!(hashtag));

fn get_hashtags(subject: &str) -> Vec<String>
{
    let mut hashtags = hashtags(subject).unwrap().1;

    hashtags.sort();
    hashtags.dedup();

    hashtags
}

named!(parse<&str, ::Task>,
    do_parse!(
        finished:
            opt!(tag_s!("x ")) >>
        priority:
            opt!(priority) >>
        finish_date:
            opt!(date) >>
        create_date:
            opt!(date) >>
        subject:
            take_till!(is_line_ending) >>
        (
            ::Task {
                subject: subject.to_owned(),
                priority: if priority.is_none() {
                    26
                } else {
                    priority.unwrap()
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
                contexts: get_contexts(subject),
                projects: get_projects(subject),
                hashtags: get_hashtags(subject),
            }
        )
    )
);

pub fn task(line: &String) -> ::Task
{
    parse(line)
        .unwrap()
        .1
}
