use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use AOC2022::get_input;

type ParsedInput = (HashMap<u8, Vec<char>>, Vec<(u8, u8, u8)>);

fn part_1(input: ParsedInput) {
    let mut stack = input.0;
    for (count, from, to) in input.1 {
        for _ in 0..count {
            let item = stack.get_mut(&from).unwrap().pop().unwrap();
            stack.get_mut(&to).unwrap().push(item)
        }
    }

    for (_, pile) in stack.drain().sorted_unstable_by(|(x, _), (y, _)| x.cmp(y)) {
        if let Some(letter) = pile.last() {
            print!("{letter}");
        }
    }
    println!();
}

fn part_2(input: ParsedInput) {
    let mut stack = input.0;
    for (count, from, to) in input.1 {
        let src_stack = stack.get_mut(&from).unwrap();
        let remove_start_index = src_stack.len() - (count as usize);
        let removed = src_stack.drain(remove_start_index..).collect::<Vec<char>>();

        stack.get_mut(&to).unwrap().extend(removed);
    }

    for (_, pile) in stack.drain().sorted_unstable_by(|(x, _), (y, _)| x.cmp(y)) {
        if let Some(letter) = pile.last() {
            print!("{letter}");
        }
    }
    println!();
}

fn get_letter_iterator(letters: &str) -> impl Iterator<Item = char> + '_ {
    letters.chars().skip(1).step_by(4)
}

fn parse_num(num: char) -> u8 {
    num.to_digit(10).unwrap() as u8
}

fn parse_header<'a>(input: &mut impl Iterator<Item = &'a str>) -> HashMap<u8, Vec<char>> {
    let mut header: HashMap<u8, Vec<char>> = HashMap::new();
    let mut header_lines = vec![];

    for x in input.by_ref() {
        if x.is_empty() {
            break;
        };

        header_lines.push(x);
    }

    let mut header_lines = header_lines.into_iter().rev();
    if let Some(line) = header_lines.next() {
        header.extend(get_letter_iterator(line).map(|num| (parse_num(num), vec![])))
    } else {
        panic!("Missing header");
    }

    for line in header_lines {
        for (index, letter) in get_letter_iterator(line)
            .enumerate()
            .map(|(x, y)| (x as u8 + 1, y))
        {
            if !letter.is_whitespace() {
                header.get_mut(&index).map(|val| val.push(letter)).unwrap();
            }
        }
    }

    header
}

fn parse(input: String) -> ParsedInput {
    let mut input = input.lines();

    let mut header = parse_header(&mut input);
    let mut moves = vec![];
    let regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for parts in input.map(|x| regex.captures(x).unwrap()) {
        let count = &parts[1].parse().unwrap();
        let from = &parts[2].parse().unwrap();
        let to = &parts[3].parse().unwrap();

        moves.push((*count, *from, *to));
    }

    (header, moves)
}

fn main() {
    let input = get_input(5);
    let input = parse(input);

    println!("PART 1");
    part_1(input.clone());
    println!("\nPART 2");
    part_2(input);
}
