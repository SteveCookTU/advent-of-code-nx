use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::str::FromStr;

pub fn run(input: &str) -> String {
    let input = input.split('\n').map(|s| s.trim_end()).collect::<Vec<_>>();
    let part1 = part1(&input);
    let part2 = part2(&input);
    format!("Part 1: {part1} fully containing assignments\n\nPart 2: {part2} partially overlapping assignments")
}

fn part1(input: &[&str]) -> u32 {
    let mut count = 0;

    for line in input {
        let (first, second) = line.split_once(',').unwrap();
        let (first_min, first_max) = first.split_once('-').unwrap();
        let (second_min, second_max) = second.split_once('-').unwrap();
        let (first_min, first_max) = (
            u32::from_str(first_min).unwrap(),
            u32::from_str(first_max).unwrap(),
        );
        let (second_min, second_max) = (
            u32::from_str(second_min).unwrap(),
            u32::from_str(second_max).unwrap(),
        );

        if (first_min >= second_min && first_max <= second_max)
            || (second_min >= first_min && second_max <= first_max)
        {
            count += 1;
        }
    }

    count
}

fn part2(input: &[&str]) -> u32 {
    let mut count = 0;

    for line in input {
        let (first, second) = line.split_once(',').unwrap();
        let (first_min, first_max) = first.split_once('-').unwrap();
        let (second_min, second_max) = second.split_once('-').unwrap();
        let (first_min, first_max) = (
            u32::from_str(first_min).unwrap(),
            u32::from_str(first_max).unwrap(),
        );
        let (second_min, second_max) = (
            u32::from_str(second_min).unwrap(),
            u32::from_str(second_max).unwrap(),
        );

        if first_min <= second_max && first_max >= second_min {
            count += 1;
        }
    }

    count
}
