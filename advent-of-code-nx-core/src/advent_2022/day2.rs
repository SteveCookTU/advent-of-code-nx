use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

pub fn run(input: &str) -> String {
    let input = input.split('\n').map(|s| s.trim_end()).collect::<Vec<_>>();
    let part1 = part1(&input);
    let part2 = part2(&input);
    format!("Part 1: Total score is {part1}\n\nPart 2: Total score is {part2}")
}

fn part1(input: &[&str]) -> u32 {
    let mut total_score = 0;
    for line in input {
        let (opponent, player) = parse_line_to_choices(line);
        let choice_score = match player {
            b'X' => 1,
            b'Y' => 2,
            b'Z' => 3,
            _ => 0,
        };
        let round_score = match opponent {
            b'A' if player == b'X' => 3,
            b'A' if player == b'Y' => 6,
            b'A' if player == b'Z' => 0,
            b'B' if player == b'X' => 0,
            b'B' if player == b'Y' => 3,
            b'B' if player == b'Z' => 6,
            b'C' if player == b'X' => 6,
            b'C' if player == b'Y' => 0,
            b'C' if player == b'Z' => 3,
            _ => 0,
        };
        total_score += choice_score + round_score;
    }
    total_score
}

fn part2(input: &[&str]) -> u32 {
    let mut total_score = 0;
    for line in input {
        let (opponent, player) = parse_line_to_choices(line);
        let round_score = match opponent {
            b'A' if player == b'X' => 3,
            b'A' if player == b'Y' => 4,
            b'A' if player == b'Z' => 8,
            b'B' if player == b'X' => 1,
            b'B' if player == b'Y' => 5,
            b'B' if player == b'Z' => 9,
            b'C' if player == b'X' => 2,
            b'C' if player == b'Y' => 6,
            b'C' if player == b'Z' => 7,
            _ => 0,
        };
        total_score += round_score;
    }
    total_score
}

fn parse_line_to_choices(line: &str) -> (u8, u8) {
    line.split_once(' ')
        .map(|(a, b)| (a.as_bytes()[0], b.as_bytes()[0]))
        .unwrap()
}
