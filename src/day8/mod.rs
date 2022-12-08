enum Direction {
    Top,
    Bottom,
    Right,
    Left,
}

#[derive(Debug)]
struct Tree {
    height: u8,
    location: (usize, usize),
}

impl Tree {
    fn new(height: u8, location: (usize, usize)) -> Self {
        Self { height, location }
    }
    fn is_visible_from(&self, field: &Field) -> Vec<Direction> {
        let mut visible = vec!();
        if field.get_up_from(self.location).iter().max().unwrap_or(&0)    < &self.height || self.location.0 == 0 {
            visible.push(Direction::Top)
        }
        if field.get_down_from(self.location).iter().max().unwrap_or(&0)  < &self.height || self.location.0 == field.width - 1 {
            visible.push(Direction::Bottom)
        }
        if field.get_left_from(self.location).iter().max().unwrap_or(&0)  < &self.height || self.location.1 == 0 {
            visible.push(Direction::Left)
        }
        if field.get_right_from(self.location).iter().max().unwrap_or(&0) < &self.height || self.location.1 == field.height - 1 {
            visible.push(Direction::Right)
        }
        return visible
    }
    fn is_visible(&self, field: &Field) -> bool {
        !self.is_visible_from(field).is_empty()
    }
    fn score(&self, field: &Field) -> u32 {
        let mut score = [0; 4];
        let mut total = 1;
        for tree in field.get_up_from(self.location).iter().rev() {
            score[0] += 1;
            if tree >= &self.height { break }
        }
        for tree in field.get_down_from(self.location) {
            score[1] += 1;
            if tree >= self.height { break }
        }
        for tree in field.get_left_from(self.location).iter().rev() {
            score[2] += 1;
            if tree >= &self.height { break }
        }
        for tree in field.get_right_from(self.location) {
            score[3] += 1;
            if tree >= self.height { break }
        }
        score.iter().for_each(|s| total *= s);
        return total
    }
}

#[derive(Debug)]
struct Field {
    field: Vec<u8>,
    width: usize,
    height: usize,
}

impl Field {
    fn from_string(field_str: String) -> Self {
        let width:  usize = field_str.split('\n').next().unwrap().len();
        let height: usize = field_str.trim().split('\n').collect::<Vec<&str>>().len();
        let mut field: Vec<u8> = vec!();
        for line in field_str.split('\n') {
            field.append(&mut line.chars().map(|charr| charr.to_digit(10).expect("NaN") as u8).collect())
        }
        return Field { field, width, height }
    }
    fn get_row(&self, index: usize) -> Vec<u8> {
        self.field.chunks(self.width).nth(index).expect("index not in range of field width").to_vec()
    }
    fn get_column(&self, index: usize) -> Vec<u8> {
        let mut return_vec = vec!();
        for row in self.field.chunks(self.width) {
            return_vec.push(row[index])
        }
        return_vec
    }
    fn get_up_from(&self, location: (usize, usize)) -> Vec<u8> {
        let column = self.get_column(location.0);
        let up_vec = column[0..location.1].to_vec();
        return up_vec
    }
    fn get_down_from(&self, location: (usize, usize)) -> Vec<u8> {
        let column = self.get_column(location.0);
        let down_vec = column[(location.1 + 1)..].to_vec();
        return down_vec
    }
    fn get_left_from(&self, location: (usize, usize)) -> Vec<u8> {
        let column = self.get_row(location.1);
        let left_vec = column[0..location.0].to_vec();
        return left_vec
    }
    fn get_right_from(&self, location: (usize, usize)) -> Vec<u8> {
        let column = self.get_row(location.1);
        let right_vec = column[(location.0 + 1)..].to_vec();
        return right_vec
    }
}

pub fn calc_solution(input: String) -> (String, String) {
    let field = Field::from_string(input);
    let mut trees: Vec<Tree> = vec!();
    let mut amount_visible = 0;
    let mut max_score = 0;

    for (loc_abs, height) in field.field.iter().enumerate() {
        let tree = Tree::new(*height, ((loc_abs % field.width) as usize, (loc_abs / field.width) as usize));
        if tree.is_visible(&field) {
            amount_visible += 1
        }
        max_score = max_score.max(tree.score(&field));
        trees.push(tree)
    }

    return (amount_visible.to_string(), max_score.to_string())
}

#[test]
fn test_solution() {
    use crate::read_input_file;
    let solution = calc_solution(read_input_file("src/day8/test"));
    assert_eq!(solution, (21.to_string(), 8.to_string()))
}
