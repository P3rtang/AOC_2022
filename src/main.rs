#![allow(non_snake_case)]
mod day2;

const INPUT_FILE: &str = "src/day2/input";

fn main() {
    let format = day2::parse_input(INPUT_FILE, true);
    let solution = day2::calc_solution(format);
    println!("{}", solution)
}
