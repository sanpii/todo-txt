#![no_main]

#[macro_use]
extern crate libfuzzer_sys;
extern crate todo_txt;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = String::from_utf8(data.to_vec()) {
        let _ = todo_txt::parser::task(&s);
    }
});
