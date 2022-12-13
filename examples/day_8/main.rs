use AOC2022::get_input;

type ParsedInput = Vec<Vec<u8>>;

fn check_row<F>(row: &[u8], y: usize, predicate: &mut F) -> bool
where
    F: FnMut(&u8, &u8) -> bool,
{
    let compared = row[y];
    let left = &row[..y];
    let right = &row[(y + 1)..];

    left.iter().all(|val| predicate(&compared, val))
        || right.iter().all(|val| predicate(&compared, val))
}

fn check_column<F>(rows: &[Vec<u8>], x: usize, y: usize, predicate: &mut F) -> bool
where
    F: FnMut(&u8, &u8) -> bool,
{
    let compared = rows[x][y];
    let mut up = rows[..x].iter().map(|row| row[y]);
    let mut down = rows[(x + 1)..].iter().map(|row| row[y]);

    up.all(|val| predicate(&compared, &val)) || down.all(|val| predicate(&compared, &val))
}

fn check_map<F>(map: &[Vec<u8>], x: usize, y: usize, mut predicate: F) -> bool
where
    F: FnMut(&u8, &u8) -> bool,
{
    check_row(&map[x], y, &mut predicate) || check_column(map, x, y, &mut predicate)
}

fn part_1(input: ParsedInput) {
    let height = input.len();
    let width = input[0].len();
    let mut exposed_trees = width * height - (width - 2) * (height - 2);

    for row in 1..(height - 1) {
        for column in 1..(width - 1) {
            if check_map(&input, row, column, |n, m| n > m) {
                exposed_trees += 1;
            }
        }
    }

    println!("{exposed_trees}");
}

fn check_left(map: &ParsedInput, x: usize, y: usize) -> u64 {
    let height = map[x][y];
    let mut counted = 0;

    for left_tree in (0..y).rev() {
        counted += 1;

        if map[x][left_tree] >= height {
            break;
        }
    }

    counted
}

fn check_right(map: &ParsedInput, x: usize, y: usize) -> u64 {
    let height = map[x][y];
    let mut counted = 0;

    for right_tree in (y + 1)..map[0].len() {
        counted += 1;

        if map[x][right_tree] >= height {
            break;
        }
    }

    counted
}

fn check_up(map: &ParsedInput, x: usize, y: usize) -> u64 {
    let height = map[x][y];
    let mut counted = 0;

    for up_tree in (0..x).rev() {
        counted += 1;

        if map[up_tree][y] >= height {
            break;
        }
    }

    counted
}

fn check_down(map: &ParsedInput, x: usize, y: usize) -> u64 {
    let height = map[x][y];
    let mut counted = 0;

    for down_tree in (x + 1)..map.len() {
        counted += 1;

        if map[down_tree][y] >= height {
            break;
        }
    }

    counted
}

fn check_all(map: &ParsedInput, x: usize, y: usize) -> u64 {
    check_left(map, x, y) * check_right(map, x, y) * check_up(map, x, y) * check_down(map, x, y)
}

fn part_2(input: ParsedInput) {
    let mut max: u64 = 0;

    for y in 0..input.len() {
        for x in 0..input[0].len() {
            let value = check_all(&input, x, y);
            max = max.max(value);
        }
    }

    println!("{max}")
}

fn parse_line(line: &str) -> Vec<u8> {
    line.chars()
        .map(|n| n.to_digit(10).expect("Failed to convert a digit") as u8)
        .collect()
}

fn parse(input: String) -> ParsedInput {
    input.lines().map(parse_line).collect()
}

fn main() {
    let input = get_input(8);
    let input = parse(input);

    println!("PART 1");
    part_1(input.clone());
    println!("\nPART 2");
    part_2(input);
}
