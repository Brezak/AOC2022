use std::iter::Take;
use itertools::Itertools;
use AOC2022::get_input;

type ParsedInput = String;

fn is_tuple_all_different(a: (char, char, char, char)) -> bool {
    a.0 != a.1 && a.0 != a.2 && a.0 != a.3 && a.1 != a.2 && a.1 != a.3 && a.2 != a.3
}

fn part_1(input: ParsedInput) {
    // Can't be used for part 2 because tuple windows only goes up to 12 elements, we need 14
    for (index, tuple) in input.chars().tuple_windows().enumerate().map(|(n, y)| (n + 4, y)) {
        if is_tuple_all_different(tuple) {
            println!("{index}");
            break;
        }
    }
}


fn part_2(input: ParsedInput) {
    const WINDOW_SIZE: usize = 14;

    let mut index = WINDOW_SIZE;
    let mut iterator = Some(input.chars());

    while let Some(mut iter) = iterator.take() {
        let mut windows = iter.clone().take(WINDOW_SIZE);
        if windows.all_unique() {
            println!("{index}");
            break;
        }
        index += 1;

        match iter.next() {
            None => iterator = None,
            Some(_) => iterator = Some(iter),
        }
    }
}

fn parse(input: String) -> ParsedInput {
    input
}

fn main() {
    let input = get_input(6);
    let input = parse(input);

    println!("PART 1");
    part_1(input.clone());
    println!("\nPART 2");
    part_2(input);
}
