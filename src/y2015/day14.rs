use std::cmp;

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> String {
    let race = 2503;
    let mut best = 0;
    for reindeer in input.trim().lines() {
        let mut dist = 0;
        let parts: Vec<&str> = reindeer.split_whitespace().collect();
        let speed: u32 = parts[3].parse().unwrap();
        let fly_time: u32 = parts[6].parse().unwrap();
        let rest_time: u32 = parts[13].parse().unwrap();
        let cycle_time = rest_time + fly_time;
        let dist_per_cycle = speed * fly_time;
        let full_cycles = race / cycle_time;
        dist += full_cycles * dist_per_cycle;

        let remaining_time = race - full_cycles * cycle_time;
        dist += cmp::min(remaining_time, fly_time) * speed;

        best = cmp::max(best, dist);
    }

    best.to_string()
}

struct Reindeer {
    next_sprint: u32,
    next_rest: u32,
    points: u32,
}

impl Reindeer {
    fn update() {}
}

fn part2(input: &str) -> String {
    "TODO".to_string()
}
