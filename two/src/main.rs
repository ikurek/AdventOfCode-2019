use std::fs::File;
use std::io::{BufRead, BufReader};

const OPCODE_ADD : i64 = 1;
const OPCODE_MUL : i64 = 2;
const OPCODE_BREAK : i64 = 99;

fn main() {
    let numbers = read_file();
    let result_one : i64 = solve_part_one(numbers.clone(), 12, 2);
    println!("Part one result: {}", result_one);

    let result_two : i64 = solve_part_two(numbers.clone());
    println!("Part two result: {}", result_two);
}

fn read_file() -> Vec<i64> {
    let filename = "input";
    let file = File::open(filename).expect("No file found");
    let reader = BufReader::new(file);

    return reader
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
}

fn solve_part_one(mut numbers: Vec<i64>, first_value: i64, second_value: i64) -> i64 {
    numbers[1] = first_value;
    numbers[2] = second_value;

    let mut position = 0;

    while numbers[position] != OPCODE_BREAK {
        let x_position = numbers[(position + 1) as usize];
        let y_position = numbers[(position + 2) as usize];
        let z_position = numbers[(position + 3) as usize];
        let x = numbers[x_position as usize];
        let y = numbers[y_position as usize];
        let z = match numbers[position] {
            OPCODE_ADD => (x + y) as i64,
            OPCODE_MUL => (x * y) as i64,
            _ => panic!("Failed OPCODE"),
        };
        
        numbers[z_position as usize] = z;
        position = position + 4;
    }

    return numbers[0];
}

fn solve_part_two(numbers: Vec<i64>) -> i64 {
    let desired = 19690720;

    for noun in 0..100 {
        for verb in 0..100 {
            if desired == solve_part_one(numbers.clone(), noun, verb) {
                return (100 * noun + verb) as i64;
            }
        }
    }

    panic!("No result found");
}
