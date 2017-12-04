# Todo-txt

[![Build Status](https://travis-ci.org/sanpii/todo-txt.svg?branch=master)](https://travis-ci.org/sanpii/todo-txt)

Parser for the [todo.txt](https://github.com/todotxt/todo.txt) format.

## Usage

Add it to your dependencies:

```toml
[dependencies]
todo-txt = "^0.1"
```

And use it:

```rust
extern crate todo_txt;

use ::std::str::FromStr;

fn main()
{
    let line = "x (A) 2016-05-20 2016-04-30 measure space for +chapelShelving @chapel due:2016-05-30";
    let task = ::todo_txt::Task::from_str(line);

    println!("{:#?}", task);
}
```
