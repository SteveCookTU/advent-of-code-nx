use alloc::collections::BTreeSet;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

pub fn run(input: &str) -> String {
    let input = input.split('\n').map(|s| s.trim_end()).collect::<Vec<_>>();
    let part1 = part1(&input);
    let part2 = part2(&input);
    format!("Part 1: The first start marker ends at index {part1}\n\nPart 2: The first message marker ends at index {part2}")
}

fn part1(input: &[&str]) -> usize {
    let line = &input[0];
    let mut processed_characters = 0;
    for (i, window) in line.as_bytes().windows(4).enumerate() {
        if window[1..].iter().any(|i| i == &window[0]) {
            continue;
        }
        if window[2..].iter().any(|i| i == &window[1]) {
            continue;
        }
        if window[2] == window[3] {
            continue;
        }
        processed_characters = i + 4;
        break;
    }

    processed_characters
}

fn part2(input: &[&str]) -> usize {
    let line = &input[0];
    let mut processed_characters = 0;
    let mut set = BTreeSet::new();
    'outer: for (i, window) in line.as_bytes().windows(14).enumerate() {
        for i in window {
            if !set.insert(*i) {
                set.clear();
                continue 'outer;
            }
        }
        processed_characters = i + 14;
        break;
    }

    processed_characters
}
