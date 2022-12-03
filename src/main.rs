#![allow(non_snake_case)]
use std::fs::OpenOptions;
use std::io::Read;

mod day3;

const INPUT_FILE: &str = "src/day3/input";

fn main() {
    let format = day3::parse_input(read_input_file(INPUT_FILE));
    let solution = day3::calc_solution(format);
    let format_part2 = day3::parse_input_part2(read_input_file(INPUT_FILE));
    let solution_part2 = day3::calc_solution(format_part2);
    println!("part1: {}\npart2: {}", solution, solution_part2)
}

fn read_input_file(file_path: &str) -> String {
    let mut input = String::new();
    let mut file = OpenOptions::new().read(true).open(file_path).unwrap();
    file.read_to_string(&mut input).unwrap();
    input
}
