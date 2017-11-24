fn is_line_ending(chr: char) -> bool
{
    chr == '\n' || chr == '\r'
}

named!(parse<&str, ::Task>,
    do_parse!(
        finished:
            opt!(tag_s!("x ")) >>
        subject:
            take_till!(is_line_ending) >>
        (
            ::Task {
                subject: subject.to_owned(),
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
