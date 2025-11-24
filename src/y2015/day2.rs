pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> String {
    let total: i32 = input
        .trim()
        .lines()
        .map(|line| {
            let mut dims: Vec<i32> = line
                .split('x')
                .map(|s| s.parse::<i32>().expect("invalid number"))
                .collect();
            dims.sort();
            let l = dims[0];
            let w = dims[1];
            let h = dims[2];
            2 * l * w + 2 * w * h + 2 * h * l + l * w
        })
        .sum();
    total.to_string()
}

fn part2(input: &str) -> String {
    "TODO".to_string()
}
