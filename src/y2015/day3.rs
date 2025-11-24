use std::collections::HashMap;
use std::collections::HashSet;

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> String {
    let mut unique_locations: HashSet<(i32, i32)> = HashSet::new();
    let mut current_position: (i32, i32) = (0, 0);
    unique_locations.insert(current_position);

    for d in input.trim().bytes() {
        match d {
            b'>' => current_position = (current_position.0 + 1, current_position.1),
            b'<' => current_position = (current_position.0 - 1, current_position.1),
            b'v' => current_position = (current_position.0, current_position.1 - 1),
            _ => current_position = (current_position.0, current_position.1 + 1),
        }
        unique_locations.insert(current_position);
    }

    unique_locations.len().to_string()
}

fn part2(input: &str) -> String {
    let mut unique_locations: HashMap<(i32, i32), (u8, u8)> = HashMap::new();
    let mut santa_pos = (0, 0);
    let mut robo_pos = (0, 0);
    unique_locations.insert((0, 0), (1, 1));

    for (i, d) in input.trim().bytes().enumerate() {
        let is_santa = i % 2 == 0;
        let mover = if is_santa {
            &mut santa_pos
        } else {
            &mut robo_pos
        };

        match d {
            b'>' => mover.0 += 1,
            b'<' => mover.0 -= 1,
            b'v' => mover.1 -= 1,
            b'^' => mover.1 += 1,
            _ => (),
        }

        let entry = unique_locations.entry(*mover).or_insert((0u8, 0u8));
        if is_santa {
            entry.0 = entry.0.saturating_add(1);
        } else {
            entry.1 = entry.1.saturating_add(1);
        }
    }

    unique_locations.len().to_string()
}
