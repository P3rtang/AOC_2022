#![allow(non_snake_case)]
#![allow(dead_code)]
use std::fs::OpenOptions;
use std::io::Read;

mod day1;
mod day2;
// mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
    day1();
    day2();
    day4();
    day5();
    day6();
    println!("-------------------")
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

fn print_solution(solution: (String, String), day: i32) {
    println!("-------------------");
    println!("Day {}", day);
    println!("part1: {}\npart2: {}", solution.0, solution.1)
}

fn day1() {
    let format = day1::parse_input("src/day1/input");
    let solution = day1::calc_solution(format);
    print_solution(solution, 1)
}

fn day2() {
    let format = day2::parse_input("src/day2/input");
    let solution = day2::calc_solution(format);
    print_solution(solution, 2)
}

fn day3() {
    todo!()
}

fn day4() {
    let format = day4::parse_input(read_input_file("src/day4/input"));
    let solution = day4::calc_solution(format);
    print_solution(solution, 4)
}

fn day5() {
    let format = day5::parse_input(read_input_file("src/day5/input"));
    let solution = day5::calc_solution(format);
    print_solution(solution, 5)
}

fn day6() {
    let solution = day6::calc_solution(read_input_file("src/day6/input"));
    print_solution(solution, 6)
}
