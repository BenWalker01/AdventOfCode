pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> String {
    input
        .trim()
        .bytes()
        .map(|b| if b == b'(' { 1i32 } else { -1i32 })
        .sum::<i32>()
        .to_string()
}

fn part2(input: &str) -> String {
    let mut floor = 0;
    for (i, b) in input.trim().bytes().enumerate() {
        if b == b'(' { floor += 1 } else { floor -= 1 }
        if floor == -1 {
            return (i + 1).to_string();
        }
    }
    "0".to_string()
}
