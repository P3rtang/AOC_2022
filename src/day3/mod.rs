use std::fs::OpenOptions;
use std::io::Read;
use std::collections::HashSet;

pub fn parse_input(input: String) -> Vec<char> {
    let mut intersecs = vec!();
    for line in input.split('\n') {
        if line.len() == 0 {
            break
        }
        let chunks = line.split_at((line.len() / 2) as usize);
        let intersec = intersection(chunks.0, chunks.1).chars().next().unwrap();
        intersecs.push(intersec)
    }
    intersecs
}

pub fn parse_input_part2(input: String) -> Vec<char> {
    let mut return_vec = vec!();
    for chunk in input.split('\n').collect::<Vec<&str>>().chunks(3) {
        if chunk.len() != 3 {
            break
        }
        let mut intersec = chunk[0].to_string();
        for line in chunk {
            let i = intersection(&intersec, line);
            intersec = i;
        }
        let mut intersec_chars = intersec.chars().collect::<Vec<char>>();
        return_vec.append(&mut intersec_chars)
    }
    return_vec
}

pub fn calc_solution(format: Vec<char>) -> i32 {
    let mut sum = 0;
    for charr in format {
        if charr.is_lowercase() {
            sum += charr.to_string().as_bytes()[0] as i32 - 96
        } else if charr.is_uppercase() {
            sum += charr.to_string().as_bytes()[0] as i32 - 38
        }
    }
    return sum
}

fn intersection(input: &str, input2: &str) -> String {
    let mut left = HashSet::new();
    for charr in input.chars() {
        left.insert(charr);
    }

    let mut right = HashSet::new();
    for charr in input2.chars() {
        right.insert(charr);
    }
    let intersec = left.intersection(&right).map(|i| *i).collect::<Vec<char>>().into_iter().collect::<String>();
    return intersec
}
