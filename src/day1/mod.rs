use std::fs::OpenOptions;
use std::io::Read;

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct Elf {
    total_calories: i32,
}

impl Elf {
    fn new(string: &str) -> Self {
        let mut items = vec!();
        for item in string.split('\n') {
            if let Ok(number) = item.parse::<i32>() {
                items.push(number);
            }
        }
        return Elf{ total_calories: items.into_iter().sum() }
    }
}

pub fn parse_input(input_path: &str) -> Vec<Elf> {
    let mut input = String::new();
    let mut file = OpenOptions::new().read(true).open(input_path).unwrap();

    file.read_to_string(&mut input).unwrap();
    
    let mut return_vec = vec!();
    for elf in input.split("\n\n") {
        return_vec.push(Elf::new(elf))
    }
    return_vec
}

pub fn calc_solution(mut elfs: Vec<Elf>, top: usize) -> i32 {
    let mut sum = 0;
    elfs.sort();
    elfs.reverse();
    
    for elf in elfs[0..top].into_iter() {
        sum += elf.total_calories
    }
    sum
}
