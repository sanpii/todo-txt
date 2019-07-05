use std::str::FromStr;

fn main() {
    let line =
        "x (A) 2016-05-20 2016-04-30 measure space for +chapelShelving @chapel due:2016-05-30";
    let task = todo_txt::Task::from_str(line);

    let json = serde_json::to_string(&task);

    println!("{}", json.expect("Unable to serialize task"));
}
