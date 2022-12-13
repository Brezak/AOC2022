use std::collections::HashSet;
use AOC2022::get_input;

type ParsedInput = Vec<(Vec<u8>, Vec<u8>)>;

fn get_common((left, right): (Vec<u8>, Vec<u8>)) -> u8 {
    let left_set: HashSet<u8> = left.into_iter().collect();
    let right_set: HashSet<u8> = right.into_iter().collect();

    let mut same = left_set.intersection(&right_set);

    let common = same
        .next()
        .unwrap_or_else(|| panic!("intersection empty left {left_set:?}, right {right_set:?}"));
    assert_eq!(same.next(), None);
    assert!(
        (*common >= b'a' && *common <= b'z') || (*common >= b'A' && *common <= b'Z'),
        "Letter is out of range"
    );

    *common
}

fn part_1(input: ParsedInput) {
    let sum: u64 = input
        .into_iter()
        .map(get_common)
        .map(|common| {
            if common.is_ascii_lowercase() {
                (common - b'a' + 1) as u64
            } else {
                (common - b'A' + 27) as u64
            }
        })
        .sum();
    println!("Sum of priorities is {sum}");
}

fn get_intersection(parts: &[(Vec<u8>, Vec<u8>)]) -> u8 {
    // This should be illegal
    parts
        .iter()
        .map(|(left, right)| left.iter().chain(right).copied().collect::<HashSet<u8>>())
        .reduce(|accumulator, next| accumulator.intersection(&next).copied().collect())
        .and_then(|x| x.into_iter().next())
        .unwrap()
}

fn part_2(input: ParsedInput) {
    let sum: u64 = input
        .chunks_exact(3)
        .map(get_intersection)
        .map(|common| {
            if common.is_ascii_lowercase() {
                (common - b'a' + 1) as u64
            } else {
                (common - b'A' + 27) as u64
            }
        })
        .sum();

    println!("{sum}");
}

fn parse(input: String) -> ParsedInput {
    input
        .lines()
        .filter(|x| !x.is_empty())
        .map(|s| {
            let (left, right) = s.as_bytes().split_at(s.len() / 2);

            (left.to_owned(), right.to_owned())
        })
        .collect()
}

fn main() {
    let input = get_input(3);
    let input = parse(input);

    println!("PART 1");
    part_1(input.clone());
    println!("\nPART 2");
    part_2(input);
}
