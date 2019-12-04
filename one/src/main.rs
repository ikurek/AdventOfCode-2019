use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let values = read_file();
    let result_one : i64 = solve_part_one(&values);

    println!("Part one result: {}", result_one);

    let result_two: i64 = solve_part_two(&values);

    println!("Part two result: {}", result_two);
}

fn read_file() -> Vec<f64> {
    let filename = "input";
    let file = File::open(filename).expect("No file found");
    let reader = BufReader::new(file);

    return reader.lines()
    .map(|line| line.unwrap().trim().parse().unwrap())
    .collect();
}

fn solve_part_one(values: &Vec<f64>) -> i64 {
    return values.iter()
    .map(|value| calculate_fuel_usage(*value))
    .sum();
}

fn calculate_fuel_usage(input: f64) -> i64 {
    return ((input/3_f64).floor() - 2_f64) as i64;
}

fn solve_part_two(values: &Vec<f64>) -> i64 {
    let mut result = 0;
    for value in values {
        let mut temporaryValue = calculate_fuel_usage(*value) as i64;
        result = result + temporaryValue;
        loop {
            temporaryValue = calculate_fuel_usage(temporaryValue as f64) as i64;
            if temporaryValue <= 0 {
                break;
            } else {
                result = result + temporaryValue
            }
        }
    }

    return result
}