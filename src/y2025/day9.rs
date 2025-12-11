use std::cmp;

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn box_size(x1: u32, y1: u32, x2: u32, y2: u32) -> u64 {
    ((x1).abs_diff(x2) + 1) as u64 * (y1.abs_diff(y2) + 1) as u64
}

fn part1(input: &str) -> String {
    let mut largest = 0;
    let data: Vec<(u32, u32)> = input
        .trim()
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(",").collect();
            (parts[0].parse().unwrap(), parts[1].parse().unwrap())
        })
        .collect();

    for (x1, y1) in &data {
        for (x2, y2) in &data {
            largest = cmp::max(largest, box_size(*x1, *y1, *x2, *y2));
        }
    }
    largest.to_string()
}

fn part2(input: &str) -> String {
    "TODO".to_string()
}
