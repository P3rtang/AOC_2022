use std::fs::OpenOptions;
use std::io::Read;

pub struct Game {
    state: (String, String),
}

impl Game {
    fn as_str(&self) -> (&str, &str) {
        return (self.state.0.as_str(), self.state.1.as_str())
    }
    fn get_points_1(&self) -> (i32, i32) {
        return match self.as_str() {
            ("A", "X") => (4, 4),
            ("A", "Y") => (1, 8),
            ("A", "Z") => (7, 3),
            ("B", "X") => (8, 1),
            ("B", "Y") => (5, 5),
            ("B", "Z") => (2, 9),
            ("C", "X") => (3, 7),
            ("C", "Y") => (9, 2),
            ("C", "Z") => (6, 6),
            _ => todo!(),
        }
    }
    fn get_points_2(&self) -> (i32, i32) {
        return match self.as_str() {
            ("A", "Y") => (4, 4),
            ("A", "Z") => (1, 8),
            ("A", "X") => (7, 3),
            ("B", "X") => (8, 1),
            ("B", "Y") => (5, 5),
            ("B", "Z") => (2, 9),
            ("C", "Z") => (3, 7),
            ("C", "X") => (9, 2),
            ("C", "Y") => (6, 6),
            _ => todo!(),
        }
    }
}

pub fn parse_input(file_path: &str) -> Vec<Game> {
    let mut guide: Vec<Game> = vec!();
    
    let mut input = String::new();
    let mut file = OpenOptions::new().read(true).open(file_path).unwrap();
    file.read_to_string(&mut input).unwrap();

    for game in input.split('\n') {
        if game.len() != 3 {
            break;
        }
        let game_vec = game.split(' ').collect::<Vec<&str>>();
        let state = (game_vec[0].to_string(), game_vec[1].to_string());
        guide.push(Game { state })
    }
    guide
}

pub fn calc_solution(guide: Vec<Game>) -> (String, String) {
    let mut score1 = 0;
    let mut score2 = 0;
    for game in guide {
        score1 += game.get_points_1().1;
        score2 += game.get_points_2().1;
    }
    return (score1.to_string(), score2.to_string())
}
