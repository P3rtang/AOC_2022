#[derive(Clone)]
struct Parser {
    input: String,
    index: usize,
    window_size: usize
}

impl Parser {
    fn new(input: String, window_size: usize) -> Self {
        return Parser { input, index: 0, window_size }
    }
}

impl Iterator for Parser {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut input = self.input.clone();
        let mut part = input.split_off(self.index);
        part.split_off(self.window_size).truncate(0);
        self.index += 1;
        return Some(part)
    }
}

fn is_unique(string: String) -> bool {
    let mut seen = vec!();
    for charr in string.chars() {
        if seen.contains(&charr) {
            return false
        }
        seen.push(charr)
    }
    return true
}

pub fn calc_solution(input: String) -> (String, String) {
    let parser1 = Parser::new(input.clone(), 4);
    let parser2 = Parser::new(input, 14);
    let mut index1 = 4;
    let mut index2 = 14;
    for window in parser1 {
        if is_unique(window) {
            break
        }
        index1 += 1;
    }
    for window in parser2 {
        if is_unique(window) {
            break
        }
        index2 += 1;
    }
    return (index1.to_string(), index2.to_string())
}

#[test]
fn test_solution() {
    use crate::read_input_file;
    let solution = calc_solution(read_input_file("src/day6/test"));
    assert_eq!(solution, ("7".to_string(), "19".to_string()))
}
