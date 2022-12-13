use std::collections::HashSet;
use std::ops::RangeInclusive;
use AOC2022::get_input;

type ParsedInput = Vec<(RangeInclusive<u64>, RangeInclusive<u64>)>;

fn is_subrange(a: &RangeInclusive<u64>, b: &RangeInclusive<u64>) -> bool {
    a.start() >= b.start() && a.end() <= b.end()
}

fn part_1(input: ParsedInput) {
    let occurrences = input.into_iter().filter(|(left, right)| is_subrange(left, right) || is_subrange(right, left)).count();

    println!("{occurrences}");
}

fn overlaps(a: &RangeInclusive<u64>, b: &RangeInclusive<u64>) -> bool {
    a.start() <= b.end() && a.end() >= b.start()
}

fn part_2(input: ParsedInput) {
    let occurrences = input.into_iter().filter(|(left, right)| overlaps(left, right)).count();

    println!("{occurrences}");
}

fn parse(input: String) -> ParsedInput {
    input
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|s| {
            let (left, right) = s.split_once(',').unwrap();
            let left = left.split_once('-').map(|(start, end)| {
                let start = start.parse().unwrap();
                let end = end.parse().unwrap();

                start..=end
            }).unwrap();

            let right = right.split_once('-').map(|(start, end)| {
                let start = start.parse().unwrap();
                let end = end.parse().unwrap();

                start..=end
            }).unwrap();

            (left, right)
        })
        .collect()
}

fn main() {
    let input = get_input(4);
    let input = parse(input);

    println!("PART 1");
    part_1(input.clone());
    println!("\nPART 2");
    part_2(input);
}
