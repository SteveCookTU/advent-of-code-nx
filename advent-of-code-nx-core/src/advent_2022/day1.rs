use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::str::FromStr;

pub fn run(input: &str) -> String {
    let input = input.split('\n').map(|s| s.trim()).collect::<Vec<_>>();
    let part1 = part1(&input);
    let part2 = part2(&input);
    format!(
        "Day 1: Max calories is {}\n\nDay 1: Sum of top 3 calories is {}",
        part1, part2
    )
}

fn part1(input: &[&str]) -> u32 {
    let mut max = 0;
    let mut current = 0;
    for line in input {
        if line.is_empty() {
            if current > max {
                max = current;
            }
            current = 0;
        } else {
            current += u32::from_str(line).unwrap();
        }
    }
    if current > max {
        max = current;
    }
    max
}

fn part2(input: &[&str]) -> u32 {
    let mut values = Vec::new();
    let mut current = 0;
    for line in input {
        if line.is_empty() {
            values.push(current);
            current = 0;
        } else {
            current += u32::from_str(line).unwrap();
        }
    }
    values.push(current);
    values.sort_by(|a, b| b.cmp(a));
    values.into_iter().take(3).sum()
}
