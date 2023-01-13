use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

pub fn run(input: &str) -> String {
    let input = input.split('\n').map(|s| s.trim_end()).collect::<Vec<_>>();
    let part1 = part1(&input);
    let part2 = part2(&input);
    format!("Part 1: {part1} visible trees\n\nPart 2: Max view distance is {part2}")
}

fn part1(input: &[&str]) -> u32 {
    let bytes = input.iter().map(|s| s.as_bytes()).collect::<Vec<_>>();

    let mut visible = 0;

    for (y, row) in bytes.iter().enumerate() {
        if y == 0 || y == bytes.len() - 1 {
            visible += row.len() as u32;
            continue;
        }
        for (x, num) in row.iter().enumerate() {
            if x == 0
                || x == row.len() - 1
                || row[..x].iter().all(|i| i < num)
                || row[(x + 1)..].iter().all(|i| i < num)
                || bytes.iter().take(y).all(|row2| row2[x] < *num)
                || bytes.iter().skip(y + 1).all(|row2| row2[x] < *num)
            {
                visible += 1;
            }
        }
    }

    visible
}

fn part2(input: &[&str]) -> usize {
    let bytes = input.iter().map(|s| s.as_bytes()).collect::<Vec<_>>();

    let mut max_view_distance = 0;

    for (y, row) in input.iter().map(|s| s.as_bytes()).enumerate() {
        if y == 0 || y == bytes.len() - 1 {
            continue;
        }
        for (x, num) in row.iter().enumerate() {
            if x == 0 || x == row.len() - 1 {
                continue;
            }
            let mut view_distance = 1;
            if let Some(x2) = row[..x].iter().rposition(|i| i >= num) {
                view_distance *= x - x2;
            } else {
                view_distance *= x;
            }
            if let Some(x2) = row[(x + 1)..].iter().position(|i| i >= num) {
                view_distance *= x2 + 1;
            } else {
                view_distance *= row.len() - 1 - x;
            }
            if let Some(y2) = bytes[..y].iter().rposition(|row2| row2[x] >= *num) {
                view_distance *= y - y2;
            } else {
                view_distance *= y;
            }
            if let Some(y2) = bytes[(y + 1)..].iter().position(|row2| row2[x] >= *num) {
                view_distance *= y2 + 1;
            } else {
                view_distance *= bytes.len() - 1 - y;
            }
            max_view_distance = max_view_distance.max(view_distance);
        }
    }

    max_view_distance
}
