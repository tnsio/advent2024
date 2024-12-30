use std::{fs::File, io::Read};

use regex::Regex;

fn main() {
    let mut input_file = File::open("./inputs/3.txt").expect("Could not open input file.");
    let mut input = String::new();

    input_file
        .read_to_string(&mut input)
        .expect("Could not read input file to string.");

    let regex = Regex::new(r"(?<mul>mul\((?<left>\d+),(?<right>\d+)\))|(?<do>do\(\))|(?<dont>don't\(\))").unwrap();

    let mut result : i64 = 0;
    let mut mult_active = true;

    for cap in regex.captures_iter(&input) {
        if let Some(_) = cap.name("mul") {
            if mult_active {
                result += &cap["left"].parse::<i64>().unwrap() * cap["right"].parse::<i64>().unwrap();
            }
        } else if let Some(_) = cap.name("do") {
            mult_active = true;
        } else if let Some(_) = cap.name("dont") {
            mult_active = false;
        }
    }

    println!("Result = {}", result);
}
