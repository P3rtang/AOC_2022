use std::collections::HashSet;

pub struct ElfPair {
    elf1: HashSet<i32>,
    elf2: HashSet<i32>,
}

impl ElfPair {
    fn from_string(string: &str) -> Self {
        let pair: Vec<&str> = string.split(',').collect();
        return ElfPair { elf1: range_to_hashset(pair[0]), elf2: range_to_hashset(pair[1]) }
    }
    fn get_union(&self) -> HashSet<i32> {
        return self.elf1.union(&self.elf2.clone()).map(|i| *i).collect::<HashSet<i32>>()
    }
    fn do_contain(&self) -> bool {
        let union = self.get_union();
        if union == self.elf1 || union == self.elf2 { return true }
        return false
    }
    fn do_overlap(&self) -> bool {
        let union = self.get_union();
        if union.len() < self.elf1.len() + self.elf2.len() { return true }
        return false
    }
}

pub fn parse_input(input: String) -> Vec<ElfPair> {
    let mut pairs = vec!();
    for line in input.split('\n') {
        if !line.contains('-') { break }
        pairs.push(ElfPair::from_string(line))
    }
    pairs
}

pub fn calc_solution(format: Vec<ElfPair>) -> (String, String) {
    let mut count = 0;
    let mut count_part2 = 0;
    for pair in &format {
        if pair.do_contain() { count += 1 }
    }
    for pair in &format {
        if pair.do_overlap() { count_part2 += 1 }
    }
    (count.to_string(), count_part2.to_string())
}

fn range_to_hashset(range: &str) -> HashSet<i32> {
    let bounds = range.split('-').collect::<Vec<&str>>();
    let set: HashSet<i32> = (bounds[0].parse().unwrap()..bounds[1].parse::<i32>().unwrap() + 1).collect();
    set
}

#[cfg(test)]
#[test]
fn range_to_hashset_test() {
    assert_eq!(range_to_hashset("2-4"), HashSet::from([2, 3, 4]));
}
#[test]
fn test_solution() {
    fn read_input_file(file_path: &str) -> String {
        use std::fs::OpenOptions;
        use std::io::Read;
        let mut input = String::new();
        let mut file = OpenOptions::new()
            .read(true)
            .open(file_path)
            .unwrap();
        file.read_to_string(&mut input).unwrap();
        input
    }
    let test_format = parse_input(read_input_file("src/day4/test"));
    let test = calc_solution(test_format);
    assert_eq!(test, (2.to_string(), 4.to_string()));
}
