# Todo-txt

[![Github actions](https://github.com/sanpii/todo-txt/workflows/.github/workflows/ci.yml/badge.svg)](https://github.com/sanpii/todo-txt/actions?query=workflow%3A.github%2Fworkflows%2Fci.yml)
[![Build Status](https://gitlab.com/sanpi/todo-txt/badges/main/pipeline.svg)](https://gitlab.com/sanpi/todo-txt/commits/main)

Parser for the [todo.txt](https://github.com/todotxt/todo.txt) format.

## Usage

Add it to your dependencies:

```toml
[dependencies]
todo-txt = "3.0"
```

And use it:

```rust
use std::str::FromStr;

fn main()
{
    let line = "x (A) 2016-05-20 2016-04-30 measure space for +chapelShelving @chapel due:2016-05-30";
    let task = todo_txt::Task::from_str(line);

    println!("{:#?}", task);
}
```

## Features

* `serde`: (De)serialization with serde. See
    [serialization.rs](examples/serialization.rs).
* `extended`: Provide a non-standard extended task type who provides common
    extra features like recurrence.
