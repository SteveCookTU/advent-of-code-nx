use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

pub fn run(input: &str) -> String {
    let input = input.split('\n').map(|s| s.trim_end()).collect::<Vec<_>>();
    let part1 = part1(&input);
    let part2 = part2(&input);
    format!("Part 1: Priority Sum is {part1}\n\nPart 2: Group Priority Sum is {part2}")
}

fn part1(input: &[&str]) -> u32 {
    let mut priority_sum = 0;
    for line in input {
        let full_bytes = line.as_bytes();
        let len = full_bytes.len() / 2;
        let first = &full_bytes[..len];
        let second = &full_bytes[len..];

        let mut in_both = *first.iter().find(|i| second.contains(i)).unwrap();

        if in_both < 97 {
            in_both -= 38;
        } else {
            in_both -= 96;
        }
        priority_sum += in_both as u32;
    }
    priority_sum
}

fn part2(input: &[&str]) -> u32 {
    let mut priority_sum = 0;
    for line in input.chunks_exact(3) {
        let first = line[0].as_bytes();
        let second = line[1].as_bytes();
        let third = line[2].as_bytes();

        let mut in_both = *first
            .iter()
            .find(|i| second.contains(i) && third.contains(i))
            .unwrap();

        if in_both < 97 {
            in_both -= 38;
        } else {
            in_both -= 96;
        }
        priority_sum += in_both as u32;
    }
    priority_sum
}
