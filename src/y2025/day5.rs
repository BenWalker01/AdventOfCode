use std::collections::HashSet;

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> String {
    let data: Vec<&str> = input.trim().split("\n\n").collect();
    let ranges = data.first().unwrap_or(&"");
    let mut ingrts: HashSet<u64> = data
        .get(1)
        .unwrap_or(&"")
        .lines()
        .filter_map(|r| r.trim().parse::<u64>().ok())
        .collect::<HashSet<u64>>();

    let mut safe: u64 = ingrts.iter().len().try_into().unwrap();
    let nums: Vec<(u64, u64)> = ranges
        .lines()
        .filter_map(|line| {
            let mut parts = line.split('-').filter_map(|p| p.trim().parse::<u64>().ok());
            match (parts.next(), parts.next()) {
                (Some(a), Some(b)) => Some((a, b)),
                _ => None,
            }
        })
        .collect();

    for (bottom, top) in nums {
        ingrts.retain(|&v| !(bottom <= v && v <= top));
    }
    safe -= ingrts.iter().len() as u64;
    safe.to_string()
}

fn part2(input: &str) -> String {
    let data: Vec<&str> = input.trim().split("\n\n").collect();
    let ranges = data.first().unwrap_or(&"");
    let mut nums: Vec<(u64, u64)> = ranges
        .lines()
        .filter_map(|line| {
            let mut parts = line.split('-').filter_map(|p| p.trim().parse::<u64>().ok());
            match (parts.next(), parts.next()) {
                (Some(a), Some(b)) => Some((a, b)),
                _ => None,
            }
        })
        .collect();
    nums.sort_by(|a, b| a.0.cmp(&b.0));
    let mut merged: Vec<(u64, u64)> = Vec::new();
    for (bottom, top) in nums.into_iter() {
        if let Some(last) = merged.last_mut() {
            if bottom <= last.1.saturating_add(1) {
                if top > last.1 {
                    last.1 = top;
                }
            } else {
                merged.push((bottom, top));
            }
        } else {
            merged.push((bottom, top));
        }
    }
    nums = merged;
    nums.iter()
        .map(|(a, b)| (*b - *a + 1) as u128)
        .sum::<u128>()
        .to_string()
}
