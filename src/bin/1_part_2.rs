use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}, iter::zip};

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

    let mut count_map_right : HashMap<i128, i128> = HashMap::new();

    for &nr in &second_list {
        count_map_right.entry(nr).and_modify(|count| { *count += 1 }).or_insert(1);
    }

    let mut simmilarity_score : i128 = 0;

    for &nr in &first_list {
        let count = *count_map_right.entry(nr).or_insert(0); 
        simmilarity_score += count * nr;
    }



    println!("{}", simmilarity_score);
}
