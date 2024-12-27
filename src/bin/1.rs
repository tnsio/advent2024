use std::{fs::File, io::{BufRead, BufReader}, iter::zip};

use regex::Regex;

fn main() {
    let two_ints_re = Regex::new(r"(\d+)\s+(\d+)\s*").unwrap();
    let input_file = File::open("./inputs/1.txt").expect("Unable to open input file");
    let input_reader = BufReader::new(input_file);
    let mut first_list : Vec<i128> = Vec::new();
    let mut second_list : Vec<i128> = Vec::new();

    for line in input_reader.lines() {
        let line = line.expect("Broken line");
        let captures = two_ints_re.captures(&line).expect("Bad input");
        first_list.push(captures.get(1).unwrap().as_str().parse().unwrap());
        second_list.push(captures.get(2).unwrap().as_str().parse().unwrap());
    }

    first_list.sort();
    second_list.sort();

    let mut result : i128 = 0;
    for (left , right) in zip(first_list.iter(), second_list.iter()) {
        result += (right - left).abs();
    }

    println!("{}", result);
}
