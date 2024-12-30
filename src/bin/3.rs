use std::{fs::File, io::Read};

use regex::Regex;

fn main() {
    let mut input_file = File::open("./inputs/3.txt").expect("Could not open input file.");
    let mut input = String::new();

    input_file
        .read_to_string(&mut input)
        .expect("Could not read input file to string.");

    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut result : i64 = 0;

    for (_, [left, right]) in regex.captures_iter(&input).map(|cap| cap.extract()) {
        result += left.parse::<i64>().unwrap() * right.parse::<i64>().unwrap();
    }

    println!("Result = {}", result);
}
