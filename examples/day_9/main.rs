use std::collections::HashMap;
use std::str::FromStr;
use AOC2022::get_input;
use crate::Move::{Down, Left, Right, Up};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Move {
    Left(u64),
    Right(u64),
    Up(u64),
    Down(u64),
}

impl Move {
    pub fn get_move(&self) -> u64 {
        match self {
            Left(n) => *n,
            Right(n) => *n,
            Up(n) => *n,
            Down(n) => *n,
        }
    }
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        return if let (Some(dir), Some(count), None) = (split.next(), split.next(), split.next()) {
            if dir.len() > 1 {
                return Err(());
            }

            let dir = dir.chars().next().unwrap();
            if let Ok(num) = count.parse() {
                match dir {
                    'L' => Ok(Left(num)),
                    'R' => Ok(Right(num)),
                    'U' => Ok(Up(num)),
                    'D' => Ok(Down(num)),
                    _ => Err(())
                }
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}

type ParsedInput = Vec<Move>;

fn update_tail_position(tail_position: (i64, i64), head_position: (i64, i64)) -> (i64, i64) {
    if head_position == tail_position {
        return tail_position;
    }

    if tail_position.0 == head_position.0 {
        if tail_position.1 > head_position.1 {
            return (tail_position.0, head_position.1 + 1);
        } else {
            return (tail_position.0, head_position.1 - 1)
        }
    } else if tail_position.1 == head_position.1 {
        if tail_position.0 > head_position.0 {
            return (head_position.0 + 1, tail_position.1);
        } else {
            return (head_position.0 - 1, tail_position.1);
        }
    } else {
        
    }
}

fn part_1(input: &ParsedInput) {
    let mut positions = HashMap::from([((0, 0),1)]);
    let mut head_position = (0, 0);
    let tail_position = (0, 0);


    for mov in input {
        for i in 0..mov.get_move() {

        }
    }
}

fn part_2(input: &ParsedInput) {

}

fn parse(input: String) -> ParsedInput {
    input.lines().map(|n| n.parse::<Move>().unwrap()).collect()
}

fn main() {
    let input = get_input(9);
    let input = parse(input);

    println!("PART 1");
    part_1(&input);
    println!("\nPART 2");
    part_2(&input);
}
