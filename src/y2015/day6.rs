pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[derive(Clone, Debug)]
struct Instruction {
    action: Action,
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

#[derive(Clone, Copy, Debug)]
enum Action {
    On,
    Off,
    Toggle,
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .filter_map(|line| {
            if line.starts_with("turn on") {
                parse_coords(&line[8..], Action::On)
            } else if line.starts_with("turn off") {
                parse_coords(&line[9..], Action::Off)
            } else if line.starts_with("toggle") {
                parse_coords(&line[7..], Action::Toggle)
            } else {
                None
            }
        })
        .collect()
}

fn parse_coords(coords_str: &str, action: Action) -> Option<Instruction> {
    let parts: Vec<&str> = coords_str.split(" through ").collect();
    if parts.len() != 2 {
        return None;
    }

    let (x1, y1) = parse_pair(parts[0].trim())?;
    let coord2_part = parts[1].split_whitespace().next()?;
    let (x2, y2) = parse_pair(coord2_part)?;

    Some(Instruction {
        action,
        x1,
        y1,
        x2,
        y2,
    })
}

fn parse_pair(s: &str) -> Option<(usize, usize)> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() == 2 {
        Some((parts[0].parse().ok()?, parts[1].parse().ok()?))
    } else {
        None
    }
}

fn part1(input: &str) -> String {
    let instructions = parse_instructions(input);
    let mut state = [false; 1000000];

    for inst in instructions {
        for y in inst.y1..=inst.y2 {
            let offset = y * 1000;
            let start = offset + inst.x1;
            let end = offset + inst.x2 + 1;
            match inst.action {
                Action::On => {
                    for v in &mut state[start..end] {
                        *v = true;
                    }
                }
                Action::Off => {
                    for v in &mut state[start..end] {
                        *v = false;
                    }
                }
                Action::Toggle => {
                    for v in &mut state[start..end] {
                        *v = !*v;
                    }
                }
            }
        }
    }

    state.iter().filter(|&&b| b).count().to_string()
}

fn part2(input: &str) -> String {
    let instructions = parse_instructions(input);
    let mut state = [0u32; 1_000_000];
    for inst in instructions {
        for y in inst.y1..=inst.y2 {
            let offset = y * 1000;
            let start = offset + inst.x1;
            let end = offset + inst.x2 + 1;
            match inst.action {
                Action::On => {
                    for v in &mut state[start..end] {
                        *v += 1;
                    }
                }
                Action::Off => {
                    for v in &mut state[start..end] {
                        *v = (*v).saturating_sub(1);
                    }
                }
                Action::Toggle => {
                    for v in &mut state[start..end] {
                        *v += 2;
                    }
                }
            }
        }
    }
    state.iter().map(|&v| v as u64).sum::<u64>().to_string()
}
