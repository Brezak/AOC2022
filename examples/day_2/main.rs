use std::cmp::Ordering;
use AOC2022::get_input;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Symbols {
    Rock,
    Paper,
    Scissors,
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum TargetResult {
    Win,
    Draw,
    Lose,
}

impl TargetResult {
    pub fn get_symbol_by_strategy(self, other: Symbols) -> Symbols {
        use Symbols::*;
        use TargetResult::*;

        match (self, other) {
            (Win, Rock) => Paper,
            (Win, Paper) => Scissors,
            (Win, Scissors) => Rock,
            (Draw, Rock) => Rock,
            (Draw, Paper) => Paper,
            (Draw, Scissors) => Scissors,
            (Lose, Rock) => Scissors,
            (Lose, Paper) => Rock,
            (Lose, Scissors) => Paper,
        }
    }
}

impl From<Symbols> for TargetResult {
    fn from(n: Symbols) -> Self {
        match n {
            Symbols::Rock => TargetResult::Lose,
            Symbols::Paper => TargetResult::Draw,
            Symbols::Scissors => TargetResult::Win,
        }
    }
}

impl PartialOrd<Self> for Symbols {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Self::cmp(self, other))
    }
}

impl Ord for Symbols {
    fn cmp(&self, other: &Self) -> Ordering {
        use Symbols::*;

        match (self, other) {
            (Rock, Rock) => Ordering::Equal,
            (Rock, Paper) => Ordering::Less,
            (Rock, Scissors) => Ordering::Greater,
            (Paper, Rock) => Ordering::Greater,
            (Paper, Paper) => Ordering::Equal,
            (Paper, Scissors) => Ordering::Less,
            (Scissors, Rock) => Ordering::Less,
            (Scissors, Paper) => Ordering::Greater,
            (Scissors, Scissors) => Ordering::Equal,
        }
    }
}

fn parse(input: String) -> Vec<(Symbols, Symbols)> {
    let mut sum = Vec::new();

    for data in input.lines().filter(|x| !x.is_empty()).map(str::as_bytes) {
        let left = match data[0] {
            b'A' => Symbols::Rock,
            b'B' => Symbols::Paper,
            b'C' => Symbols::Scissors,
            e => panic!("Invalid symbol in the left position {}", e as char),
        };

        let right = match data[2] {
            b'X' => Symbols::Rock,
            b'Y' => Symbols::Paper,
            b'Z' => Symbols::Scissors,
            e => panic!("Invalid symbol in the right position {}", e as char),
        };

        sum.push((left, right));
    }

    sum
}

fn part_1(input: Vec<(Symbols, Symbols)>) {
    let sum: i64 = input
        .into_iter()
        .map(|(opponent, me)| {
            let shape_score = match me {
                Symbols::Rock => 1,
                Symbols::Paper => 2,
                Symbols::Scissors => 3,
            };

            let round_result = match opponent.cmp(&me) {
                Ordering::Less => 6,
                Ordering::Equal => 3,
                Ordering::Greater => 0,
            };

            shape_score + round_result
        })
        .sum();

    println!("Strategy result: {}", sum);
}

fn calculate_round((opponent, target): (Symbols, TargetResult)) -> i64 {
    let round_result = match target {
        TargetResult::Win => 6,
        TargetResult::Draw => 3,
        TargetResult::Lose => 0,
    };

    let me = target.get_symbol_by_strategy(opponent);

    let shape_score = match me {
        Symbols::Rock => 1,
        Symbols::Paper => 2,
        Symbols::Scissors => 3,
    };

    shape_score + round_result
}

fn part_2(input: Vec<(Symbols, Symbols)>) {
    let sum: i64 = input
        .into_iter()
        .map(|(a, b)| (a, b.into()))
        .map(calculate_round)
        .sum();

    println!("Strategy result: {}", sum);
}

fn main() {
    let input = get_input(2);
    let input = parse(input);

    println!("PART 1");
    part_1(input.clone());
    println!("\nPART 2");
    part_2(input);
}
