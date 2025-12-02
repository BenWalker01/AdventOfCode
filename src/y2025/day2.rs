pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn is_valid(input: &str) -> bool {
    let (a, b) = input.split_at(input.len() / 2);
    a != b
}

fn is_valid2(input: &str) -> bool {
    for split in 2..=input.len() {
        if input.len().is_multiple_of(split) {
            let subs = input
                .as_bytes()
                .chunks(input.len() / split)
                .map(str::from_utf8)
                .collect::<Result<Vec<&str>, _>>()
                .unwrap();
            let first = subs[0];
            if subs.iter().all(|&item| item == first) {
                return false;
            }
        }
    }
    true
}

fn part1(input: &str) -> String {
    let mut sum: u64 = 0;
    for id in input.trim().split(',') {
        let mut ids = id.split('-');
        let a = ids.next().unwrap().parse::<u64>().unwrap();
        let b = ids.next().unwrap().parse::<u64>().unwrap();
        for test in a..=b {
            if !is_valid(&test.to_string()) {
                sum += test;
            }
        }
    }

    sum.to_string()
}

fn part2(input: &str) -> String {
    let mut sum: u64 = 0;
    for id in input.trim().split(',') {
        let mut ids = id.split('-');
        let a = ids.next().unwrap().parse::<u64>().unwrap();
        let b = ids.next().unwrap().parse::<u64>().unwrap();
        for test in a..=b {
            if !is_valid2(&test.to_string()) {
                sum += test;
            }
        }
    }

    sum.to_string()
}
