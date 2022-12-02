#![allow(non_snake_case)]
mod day1;

const INPUT_FILE: &str = "src/day1/input";

fn main() {
    let format = day1::parse_input(INPUT_FILE);
    let solution = day1::calc_solution(format, 3);
    println!("{}", solution)
}
