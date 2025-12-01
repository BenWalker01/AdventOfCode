pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> String {
    let mut count_zero = 0;
    let mut position = 50;
    let num = 100; // 0-99

    for line in input.trim().lines() {
        let direction: i32 = if line.starts_with('L') { -1 } else { 1 };
        let amount: i32 = line[1..].parse::<i32>().unwrap();
        if direction == -1 {
            position += num;
        }
        position += direction * amount;
        position %= num;
        if position == 0 {
            count_zero += 1;
        }
    }

    count_zero.to_string()
}

fn part2(input: &str) -> String {
    let mut count_zero = 0;
    let mut position = 50;
    let num = 100; // 0-99

    for line in input.trim().lines() {
        let direction: i32 = if line.starts_with('L') { -1 } else { 1 };
        let amount: i32 = line[1..].parse::<i32>().unwrap();
        let old = position;

        if direction == 1 {
            count_zero += (old + amount) / num;
        } else if old == 0 {
            count_zero += amount / num;
        } else if amount >= old {
            count_zero += 1 + (amount - old) / num;
        }

        position = (old + direction * amount).rem_euclid(num);
    }

    count_zero.to_string()
}
