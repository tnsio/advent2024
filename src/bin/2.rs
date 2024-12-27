use std::{fs::File, io::{BufRead, BufReader}};

#[derive(PartialEq, Clone, Copy)]
enum Order {
    Ascending,
    Descending,
}

const MIN_DIFF : i64 = 1;
const MAX_DIFF : i64 = 3;
fn main() {
    let input_file = File::open("inputs/2.txt").expect("Could not open input file");
    let input_reader = BufReader::new(input_file);

    let mut nr_bad_reports : i64 = 0;
    let mut nr_reports : i64 = 0;

    for line in input_reader.lines() {
        nr_reports += 1;
        let line = line.expect("Broken line");

        let report = line.split(" ").map(|nr_str| { nr_str.parse::<i64>().unwrap()});

        let mut order : Option<Order> = None;
        let mut previous : Option<i64> = None;

        for nr in report {
            if let Some(prev) = previous {
                let diff = (prev - nr).abs();
                if diff > MAX_DIFF || diff < MIN_DIFF {
                    nr_bad_reports += 1;
                    break;
                }

                let new_order : Order;
                if prev < nr {
                    new_order = Order::Ascending;
                } else {
                    new_order = Order::Descending
                }

                if let Some(prev_order) = order {
                    if prev_order != new_order {
                        nr_bad_reports += 1;
                        break;
                    }

                }

                order = Some(new_order);
            }

            previous = Some(nr);
        }
    }

    println!("Nr. of good reports = {}", nr_reports - nr_bad_reports);
}
