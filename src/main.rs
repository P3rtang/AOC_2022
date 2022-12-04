#![allow(non_snake_case)]
use std::fs::OpenOptions;
use std::io::Read;

mod day4;

const INPUT_FILE: &str = "src/day4/input";

fn main() {
    let format = day4::parse_input(read_input_file(INPUT_FILE));
    let solution = day4::calc_solution(format);
    println!("part1: {}\npart2: {}", solution.0, solution.1)
}

fn read_input_file(file_path: &str) -> String {
    let mut input = String::new();
    let mut file = OpenOptions::new()
        .read(true)
        .open(file_path)
        .unwrap();
    file.read_to_string(&mut input).unwrap();
    input
}
