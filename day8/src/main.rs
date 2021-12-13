use std::env;
use std::fs;

fn parse_input(raw_input: String) -> Vec<(Vec<String>, Vec<String>)> {
    raw_input
        .lines()
        .map(|line| {
            let mut parts = line.split(" | ");
            let patterns = parts
                .next()
                .unwrap()
                .split(" ")
                .map(|s| s.to_string())
                .collect();
            let digits = parts
                .next()
                .unwrap()
                .split(" ")
                .map(|s| s.to_string())
                .collect();

            (patterns, digits)
        })
        .collect()
}

fn count_easy_digits(notes: &Vec<(Vec<String>, Vec<String>)>) -> u32 {
    let mut count = 0;
    for note in notes.iter() {
        for digits in note.1.iter() {
            let len = digits.len();
            if len == 2 || len == 3 || len == 4 || len == 7 {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let mut file_name: String = "input.txt".to_string();

    let mut args = env::args();
    if args.len() >= 2 {
        file_name = args.nth(1).unwrap();
    }

    let notes = parse_input(
        fs::read_to_string(&file_name)
            .expect(format!("Could not read input file {}", &file_name).as_ref()),
    );
    println!("Day 8: Easy digits count = {}", count_easy_digits(&notes));
}
