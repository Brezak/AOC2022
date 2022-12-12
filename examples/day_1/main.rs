use AOC2022::get_input;

fn part_1(input: Vec<Vec<i64>>) {
    let max: i64 = input
        .iter()
        .map(|elf| elf.iter().sum())
        .max()
        .unwrap();

    println!("Max Elf: {max}");
}

fn part_2(input: Vec<Vec<i64>>) {
    let mut summed: Vec<i64> = input.into_iter().map(|elf| elf.into_iter().sum()).collect();
    summed.sort_unstable(); // There are better ways to do this but this solution is simplest

    println!("{}", summed[(summed.len() - 3)..].iter().sum::<i64>());
}

fn parse(input: String) -> Vec<Vec<i64>> {
    let mut collector = Vec::new();
    let mut total = Vec::new();

    for i in input.split('\n') {
        if i.is_empty() {
            let temp = collector;
            collector = Vec::new();
            total.push(temp);
        } else {
            let parsed = i.parse().unwrap();

            collector.push(parsed);
        }
    }

    total
}

fn main() {
    let input = get_input(1);
    let input = parse(input);

    println!("PART 1");
    part_1(input.clone());
    println!("\nPART 2");
    part_2(input);
}
