#![allow(non_snake_case)]
use std::fs::OpenOptions;
use std::io::Read;

mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    day1();
    day4();
    println!("---------------")
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

fn print_solution(solution: (i32, i32), day: i32) {
    println!("---------------");
    println!("Day {}", day);
    println!("part1: {}\npart2: {}", solution.0, solution.1)
}

fn day1() {
    let format = day1::parse_input("src/day1/input");
    let solution = day1::calc_solution(format);

    print_solution(solution, 1)
}

fn day2() {
    todo!()
}

fn day3() {
    todo!()
}

fn day4() {
    let format = day4::parse_input(read_input_file("src/day4/input"));
    let solution = day4::calc_solution(format);
    print_solution(solution, 4)
}
