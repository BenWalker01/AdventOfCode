pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn look_and_say(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars();
    if let Some(mut prev) = chars.next() {
        let mut count: usize = 1;
        for c in chars {
            if c == prev {
                count += 1;
            } else {
                result.push_str(&count.to_string());
                result.push(prev);
                prev = c;
                count = 1;
            }
        }
        result.push_str(&count.to_string());
        result.push(prev);
    }
    result
}

fn part1(input: &str) -> String {
    let mut current = input.trim().to_string();
    for _ in 0..40 {
        current = look_and_say(&current);
    }
    current.len().to_string()
}

fn part2(input: &str) -> String {
    let mut current = input.trim().to_string();
    for _ in 0..50 {
        current = look_and_say(&current);
    }
    current.len().to_string()
}
