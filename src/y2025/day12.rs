use std::collections::HashMap;

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> String {
    let mut total = 0;
    let data: Vec<&str> = input.trim().split("\n\n").collect();
    let schema: Vec<&str> = data[data.len() - 1].lines().collect();
    let mut presents = HashMap::new();
    for (idx, p) in data[..data.len() - 1].iter().enumerate() {
        let count = p.chars().filter(|&c| c == '#').count();
        presents.insert(idx, count);
    }
    for region in schema {
        let info: Vec<&str> = region.split_ascii_whitespace().collect();
        let dims: Vec<usize> = info[0]
            .trim_end_matches(':')
            .split('x')
            .map(|s| s.parse().unwrap())
            .collect();
        let (width, height) = (dims[0], dims[1]);
        let area = width * height;
        let mut total_cells = 0;
        for (i, val) in info[1..].iter().enumerate() {
            let count = val.parse::<usize>().unwrap_or(0);
            total_cells += presents[&i] * count;
        }
        if total_cells > area {
            continue;
        }
        let density = (total_cells as f64) / (area as f64);

        if density < 0.75 {
            total += 1;
        }
    }
    total.to_string()
}

fn part2(input: &str) -> String {
    "TODO".to_string()
}
