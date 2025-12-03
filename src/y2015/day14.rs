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
    sprint_left: u32,
    rest_left: u32,
    points: u32,
    distance: u32,
    speed: u32,
    fly_time: u32,
    rest_time: u32,
}

impl Reindeer {
    fn update(&mut self) {
        if self.sprint_left > 0 {
            self.distance += self.speed;
            self.sprint_left -= 1;
            if self.sprint_left == 0 {
                self.rest_left = self.rest_time;
            }
        } else if self.rest_left > 0 {
            self.rest_left -= 1;
            if self.rest_left == 0 {
                self.sprint_left = self.fly_time;
            }
        }
    }
    fn new(line: &str) -> Self {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let speed: u32 = parts[3].parse().unwrap();
        let fly_time: u32 = parts[6].parse().unwrap();
        let rest_time: u32 = parts[13].parse().unwrap();
        Reindeer {
            sprint_left: fly_time,
            rest_left: 0,
            points: 0,
            distance: 0,
            speed,
            fly_time,
            rest_time,
        }
    }
}

fn part2(input: &str) -> String {
    let race = 2503;
    let mut reindeers: Vec<Reindeer> = input.trim().lines().map(Reindeer::new).collect();

    for _ in 0..race {
        for r in reindeers.iter_mut() {
            r.update();
        }
        let max_dist = reindeers.iter().map(|r| r.distance).max().unwrap_or(0);
        for r in reindeers.iter_mut() {
            if r.distance == max_dist {
                r.points += 1;
            }
        }
    }

    let best = reindeers.iter().map(|r| r.points).max().unwrap_or(0);
    best.to_string()
}
