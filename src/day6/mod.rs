#[derive(Clone)]
struct Parser<const T: usize> {
    input: Vec<u8>,
    index: usize,
}

impl<const T: usize> Parser<T> {
    fn new(input: String) -> Self {
        return Parser { input: input.chars().map(|c| c as u8).collect::<Vec<u8>>().try_into().unwrap(), index: 0 }
    }
}

impl<const T: usize> Iterator for Parser<T> {
    type Item = [u8; T];

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        return Some(self.input[self.index - 1..self.index - 1 + T].try_into().unwrap())
    }
}

fn is_unique<const T: usize> (window: [u8; T]) -> bool {
    let mut seen = [0b0; 32];
    for charr in window.iter().map(|c| c % 32) {
        if seen[charr as usize] == 1 { return false }
        seen[charr as usize] = 0b1;
    }
    return true
}

pub fn calc_solution(input: String) -> (String, String) {
    let parser1 = Parser::<4>::new(input.clone());
    let parser2 = Parser::new(input);
    let mut index1 = 4;
    let mut index2 = 14;
    for window in parser1 {
        if is_unique::<4>(window) {
            break
        }
        index1 += 1;
    }
    for window in parser2 {
        if is_unique::<14>(window) {
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
