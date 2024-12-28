use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(PartialEq, Clone, Copy)]
enum Order {
    Ascending,
    Descending,
}

const MIN_DIFF: i64 = 1;
const MAX_DIFF: i64 = 3;

fn vec_with_remove_at_index<T>(report: &Vec<T>, remove_index: usize) -> Vec<T>
where
    T: Clone,
{
    report
        .iter()
        .enumerate()
        .filter_map(|(el_index, el)| {
            if remove_index != el_index {
                Some(el.clone())
            } else {
                None
            }
        })
        .collect()
}

fn check_report(report: &Vec<i64>) -> Result<(), usize> {
    let mut order: Option<Order> = None;
    let mut previous: Option<i64> = None;

    for (index, &nr) in report.iter().enumerate() {
        if let Some(prev) = previous {
            let diff = (prev - nr).abs();
            if diff > MAX_DIFF || diff < MIN_DIFF {
                return Err(index);
            }

            let new_order: Order;
            if prev < nr {
                new_order = Order::Ascending;
            } else {
                new_order = Order::Descending;
            }

            if let Some(prev_order) = order {
                if prev_order != new_order {
                    return Err(index);
                }
            }

            order = Some(new_order);
        }

        previous = Some(nr);
    }

    Ok(())
}

fn main() {
    let input_file = File::open("inputs/2.txt").expect("Could not open input file");
    let input_reader = BufReader::new(input_file);

    let mut nr_bad_reports: i64 = 0;
    let mut nr_reports: i64 = 0;

    for line in input_reader.lines() {
        nr_reports += 1;
        let line = line.expect("Broken line");

        let report: Vec<i64> = line
            .split(" ")
            .map(|nr_str| nr_str.parse::<i64>().unwrap())
            .collect();

        let check_result = check_report(&report);
        if check_result.is_ok() {
            continue;
        }

        let err_index = check_result.unwrap_err();
        if check_report(&vec_with_remove_at_index(&report, err_index)).is_ok()
            || check_report(&vec_with_remove_at_index(&report, err_index - 1)).is_ok()
            || (err_index >= 2
                && check_report(&vec_with_remove_at_index(&report, err_index - 2)).is_ok())
        {
            continue;
        }

        nr_bad_reports += 1;
    }

    println!("Nr. of good reports = {}", nr_reports - nr_bad_reports);
}
