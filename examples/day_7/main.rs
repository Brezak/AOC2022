mod file_system;

use crate::file_system::{FileSystem, FileSystemObject};
use AOC2022::get_input;

type ParsedInput = FileSystem;

fn recurse_part1(obj: &FileSystemObject) -> u64 {
    const MAX_SIZE: u64 = 100000;
    match obj {
        FileSystemObject::Directory { contents, .. } => {
            let size = obj.calculate_size();
            if size > MAX_SIZE {
                contents.iter().map(recurse_part1).sum()
            } else {
                size + contents.iter().map(recurse_part1).sum::<u64>()
            }
        }
        FileSystemObject::File { .. } => 0,
    }
}

fn part_1(input: ParsedInput) {
    let size = recurse_part1(&input.root);

    println!("{size}");
}

fn recurse_part2(obj: &FileSystemObject, target: &mut u64, needed_space: u64) {
    let size = obj.calculate_size();
    if size < needed_space {
        return; // There's no point in recursing further since the size can only get lower the further in you go
    }

    if size < *target {
        *target = size;
    }

    match obj {
        FileSystemObject::Directory { contents, .. } => contents
            .iter()
            .for_each(|x| recurse_part2(x, target, needed_space)),
        FileSystemObject::File { .. } => (),
    }
}

fn part_2(input: ParsedInput) {
    const TOTAL_SPACE: u64 = 70000000;
    const UPDATE_SIZE: u64 = 30000000;
    let used_space = input.get_size();
    let needed_space = UPDATE_SIZE - (TOTAL_SPACE - used_space);

    let mut target = u64::MAX;

    recurse_part2(&input.root, &mut target, needed_space);

    println!("{target}");
}

fn parse_filesystem<'a>(lines: &mut impl Iterator<Item = &'a str>, current: &mut FileSystemObject) {
    while let Some(line) = lines.next() {
        if line.starts_with('$') {
            let command = line.strip_prefix("$ ").unwrap();
            if let Some(dir) = command.strip_prefix("cd ") {
                if dir == ".." {
                    return;
                }

                let found = current.find_dir_mut(dir);
                parse_filesystem(lines, found);
            }

            continue;
        }

        if let Some(name) = line.strip_prefix("dir ") {
            current.add_dir(name.to_owned());
        } else {
            let mut parts = line.split_whitespace();
            let size = parts.next().unwrap().parse::<u64>().unwrap();
            let name = parts.next().unwrap().to_owned();
            current.add_file(name, size);
        }
    }
}

fn parse(input: String) -> ParsedInput {
    let mut files = FileSystem::default();

    // Skip the first cd to root
    let mut lines = input.lines().skip(1);
    parse_filesystem(&mut lines, &mut files.root);

    files
}

fn main() {
    let input = get_input(7);
    let input = parse(input);

    println!("PART 1");
    part_1(input.clone());
    println!("\nPART 2");
    part_2(input);
}
