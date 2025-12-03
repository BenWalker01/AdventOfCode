pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> String {
    let mut sum = 0;
    for bank in input.trim().lines() {
        let b: Vec<u32> = bank
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| c.to_digit(10).unwrap())
            .collect();

        let max_bat = b[0..b.len() - 1].iter().max().unwrap();
        let index = b.iter().position(|r| r == max_bat).unwrap();
        let next_large = b[index + 1..].iter().max().unwrap();
        let joltage = max_bat * 10 + next_large;
        sum += joltage;
    }

    sum.to_string()
}

fn part2(input: &str) -> String {
    let mut total: u64 = 0;
    for bank in input.trim().lines() {
        let b: Vec<u32> = bank
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        let mut joltage: Vec<u64> = Vec::new();
        let mut prev = 0;
        for i in (0..12).rev() {
            let best = b[prev..b.len() - i].iter().max().unwrap();
            let index = b[prev..].iter().position(|r| r == best).unwrap();
            prev += index + 1;
            joltage.push((*best).into());
        }
        total += joltage.iter().fold(0, |acc, elem| acc * 10 + elem);
    }

    total.to_string()
}
