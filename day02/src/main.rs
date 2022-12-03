use core::panic;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::fmt;

fn main() {
    //first();
    second();
}

fn first() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut total_score = 0;
        for line in lines {
            if let Ok(str) = line {
                let enemy_shape = get_shape(&str.chars().next().unwrap());
                let my_shape = get_shape(&str.chars().nth(2).unwrap());
                let match_score = winscore(&enemy_shape, &my_shape) + shapescore(&my_shape);
                println!("{} => {} ({} vs {})", match_score, str, enemy_shape, my_shape);
                total_score += match_score;
            }
        }
        println!("Total score: {}", total_score);
    }
}

fn second() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut total_score = 0;
        for line in lines {
            if let Ok(str) = line {
                let enemy_shape = get_shape(&str.chars().next().unwrap());
                let expected_outcome = get_outcome(&str.chars().nth(2).unwrap());
                let my_shape = get_needed_shape(enemy_shape.clone(), &expected_outcome);
                let match_score = winscore(&enemy_shape, &my_shape) + shapescore(&my_shape);
                println!("{} => Enemy: {}, Expected: {}, So my shape: {}. Match score: {}", str, enemy_shape, expected_outcome, my_shape, match_score);
                total_score += match_score;
            }
        }
        println!("Total score: {}", total_score);
    }

}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors
}

impl fmt::Display for Shape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Shape::Rock => write!(f, "Rock"),
            Shape::Paper => write!(f, "Paper"),
            Shape::Scissors => write!(f, "Scissors"),
        }
    }
}

fn get_shape(c: &char) -> Shape {
    match c {
        'A' | 'X' => return Shape::Rock,
        'B' | 'Y' => return Shape::Paper,
        'C' | 'Z' => return Shape::Scissors,
        _ => panic!("oh no")
    }
}

fn get_shape_from_score(shapescore: i32) -> Shape {
    match shapescore {
        1 => return Shape::Rock,
        2 => return Shape::Paper,
        3 => return Shape::Scissors,
        _ => panic!("oh no")
    }
}

fn shapescore(shape: &Shape) -> i32 {
    match shape {
        Shape::Rock => return 1,
        Shape::Paper => return 2,
        Shape::Scissors => return 3
    }
}

fn winscore(enemy_shape : &Shape, my_shape : &Shape) -> i32 {
    if enemy_shape == my_shape {
        return 3; // Draw
    } else if shapescore(&enemy_shape) + 1 == shapescore(&my_shape) ||
        shapescore(&enemy_shape) - 2 == shapescore(&my_shape) {
        return 6; // I won
    } else {
        return 0; // I lost
    }
}

enum Outcome {
    Lose,
    Draw,
    Win
}

impl fmt::Display for Outcome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Outcome::Lose => write!(f, "Lose"),
            Outcome::Draw => write!(f, "Draw"),
            Outcome::Win => write!(f, "Win"),
        }
    }
}

fn get_outcome(c: &char) -> Outcome {
    match c {
        'X' => return Outcome::Lose,
        'Y' => return Outcome::Draw,
        'Z' => return Outcome::Win,
        _ => panic!("oh no")
    }
}

fn get_needed_shape(enemy_shape : Shape, expected_outcome : &Outcome) -> Shape {
    match expected_outcome {
        Outcome::Lose => {
            if enemy_shape == Shape::Rock {
                return Shape::Scissors;
            } else {
                return get_shape_from_score(shapescore(&enemy_shape)-1);
            }
        },
        Outcome::Draw => return enemy_shape,
        Outcome::Win => {
            if enemy_shape == Shape::Scissors {
                return Shape::Rock;
            } else {
                return get_shape_from_score(shapescore(&enemy_shape)+1);
            }
        },
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}