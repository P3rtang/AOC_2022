use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Supply {
    supply: HashMap<usize, Pile>,
}

impl Supply {
    fn new(input: String) -> Self {
        let mut supply: HashMap<usize, Pile> = HashMap::new();

        let split: Vec<&str> = input.split('\n').collect();
        let size = split.last().unwrap().replace(" ", "").len();
        for index in 0..size {
            let mut pile_vec = vec!();
            for line in &split[0..split.len() - 1] {
                let charr = line.chars().nth(4 * index + 1).unwrap();
                if charr != ' ' {
                    pile_vec.push(charr);
                }
            }
            pile_vec.reverse();
            let pile = Pile { pile: pile_vec };
            supply.insert(index, pile);
        }
        return Supply { supply }
    }
    fn swap_top(&mut self, index1: usize, index2: usize, repeat: usize) {
        let mut pile1 = self.supply[&index1].clone();
        let mut pile2 = self.supply[&index2].clone();
        for _ in 0..repeat {
            if let Some(charr) = pile1.pop(1)[0] {
                pile2.add(charr)
            }
        }
        self.supply.insert(index1, pile1);
        self.supply.insert(index2, pile2);
    }
    fn swap(&mut self, index1: usize, index2: usize, size: usize) {
        let mut pile1 = self.supply[&index1].clone();
        let mut pile2 = self.supply[&index2].clone();
        for charr in pile1.pop(size) {
            pile2.add(charr.unwrap())
        }
        self.supply.insert(index1, pile1);
        self.supply.insert(index2, pile2);
    }
}

#[derive(Debug, Clone)]
struct Pile {
    pile: Vec<char>,
}

impl Pile {
    fn new(pile: Vec<char>) -> Self {
        return Pile { pile }
    }
    fn pop(&mut self, amount: usize) -> Vec<Option<char>> {
        let mut top = vec!();
        for _ in 0..amount {
            top.push(self.pile.pop())
        }
        top.reverse();
        top
    }
    fn add(&mut self, charr: char) {
        self.pile.push(charr)
    }
    fn peek(&self) -> char {
        if self.pile.len() == 0 {
            println!("empty pile");
            return ' '
        }
        return self.pile[self.pile.len() - 1]
    }
}

pub struct Instructions {
    instructions: Vec<(i32, i32, i32)>,
}

impl Instructions {
    fn from_string(input: String) -> Self {
        let mut instructions = vec!();
        for line in input.split('\n') {
            if line.len() < 10 {
                break
            }
            let mut line_instruction = vec!();
            for part in line.split(' ') {
                if let Ok(num) = part.parse::<i32>() {
                    line_instruction.push(num)
                }
            }
            instructions.push((line_instruction[0], line_instruction[1] - 1, line_instruction[2] - 1))
        }
        return Instructions { instructions }
    }
}

pub fn parse_input(input: String) -> (Supply, Instructions) {
    let mut parts = input.split("\n\n");
    let supply = Supply::new(parts.next().unwrap().to_string());
    let instruction = Instructions::from_string(parts.next().unwrap().to_string());
    return (supply, instruction)
}

pub fn calc_solution(format: (Supply, Instructions)) -> (String, String) {
    let mut supply = format.0.clone();
    let instructions = format.1;
    for instr in &instructions.instructions {
        supply.swap_top(instr.1 as usize, instr.2 as usize, instr.0 as usize);
    }
    let mut tops1 = String::new();
    for index in 0..supply.supply.len() {
        tops1.push(supply.supply[&index].peek())
    }
    let mut supply2 = format.0;
    for instr in instructions.instructions {
        supply2.swap(instr.1 as usize, instr.2 as usize, instr.0 as usize)
    }
    let mut tops2 = String::new();
    for index in 0..supply.supply.len() {
        tops2.push(supply2.supply[&index].peek())
    }
    return (tops1, tops2)
}

#[test]
fn test_solution() {
    use std::fs::OpenOptions;
    use std::io::Read;
    fn read_input_file(file_path: &str) -> String {
        let mut input = String::new();
        let mut file = OpenOptions::new()
            .read(true)
            .open(file_path)
            .unwrap();
        file.read_to_string(&mut input).unwrap();
        input
    }
    let format = parse_input(read_input_file("src/day5/test"));
    let solution = calc_solution(format);
    assert_eq!(("CMZ".to_string(), "MCD".to_string()), solution)
}
