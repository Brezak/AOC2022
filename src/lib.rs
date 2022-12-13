use reqwest::header::COOKIE;
use std::env::args;
use std::fmt::Display;
use std::fs;
use std::path::Path;

pub fn get_input(day: u8) -> String {
    let id = args().nth(1).unwrap();

    get_input_with_session_id(day, id)
}

pub fn get_input_with_session_id(day: u8, session_id: String) -> String {
    let target_file = format!("input_day_{day}.txt");

    if Path::new(&target_file).exists() {
        return fs::read_to_string(target_file).unwrap();
    }

    let response = reqwest::blocking::Client::default()
        .get(format!("https://adventofcode.com/2022/day/{day}/input"))
        .header(COOKIE, format!("session={session_id}"))
        .send()
        .unwrap();

    if !response.status().is_success() {
        panic!("{}", response.status())
    }

    let text = response.text().unwrap();

    fs::write(target_file, &text).expect("Failed to write to file");

    text
}

pub fn print_iterator_as_array<I, T>(iter: I)
where
    I: IntoIterator<Item = T>,
    T: Display,
{
    let mut printed_first = false;
    print!("[");
    for i in iter {
        if printed_first {
            print!(", ");
        }

        printed_first = true;
        print!("{i}");
    }
    println!("]");
}
