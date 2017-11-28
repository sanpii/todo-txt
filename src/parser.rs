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

named!(parse<&str, ::Task>,
    do_parse!(
        finished:
            opt!(tag_s!("x ")) >>
        created:
            opt!(date) >>
        subject:
            take_till!(is_line_ending) >>
        (
            ::Task {
                subject: subject.to_owned(),
                created: created,
                finished: finished.is_some(),
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
