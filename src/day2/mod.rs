use std::fs::OpenOptions;
use std::io::Read;

pub struct Game {
    state: (String, String),
    points: (i32, i32),
}

pub fn parse_input(file_path: &str, win_state: bool) -> Vec<Game> {
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
        if win_state {
            guide.push(Game { state, points: calc_points_2(game) })
        } else {
            guide.push(Game { state, points: calc_points_1(game) })
        }
    }
    guide
}

fn calc_points_1(game_str: &str) -> (i32, i32) {
    return match game_str {
        "A X" => (4, 4),
        "A Y" => (1, 8),
        "A Z" => (7, 3),
        "B X" => (8, 1),
        "B Y" => (5, 5),
        "B Z" => (2, 9),
        "C X" => (3, 7),
        "C Y" => (9, 2),
        "C Z" => (6, 6),
        _ => todo!(),
    }
}

fn calc_points_2(game_str: &str) -> (i32, i32) {
    return match game_str {
        "A Y" => (4, 4),
        "A Z" => (1, 8),
        "A X" => (7, 3),
        "B X" => (8, 1),
        "B Y" => (5, 5),
        "B Z" => (2, 9),
        "C Z" => (3, 7),
        "C X" => (9, 2),
        "C Y" => (6, 6),
        _ => todo!(),
    }
}

pub fn calc_solution(guide: Vec<Game>) -> i32 {
    let mut score = 0;
    for game in guide {
        score += game.points.1
    }
    return score
}
