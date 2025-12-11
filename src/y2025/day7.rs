use std::collections::HashMap;

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> String {
    let data: Vec<&str> = input.trim().lines().collect();
    let mut beams = vec![false; data[0].len()];
    let mut splits = 0;
    let start = data[0].find('S').unwrap();
    beams[start] = true;
    for line in data.iter().skip(1) {
        let ports: Vec<char> = line.chars().collect();
        let mut new_beams = vec![false; beams.len()];
        for (i, &beam) in beams.iter().enumerate() {
            if beam && i < ports.len() && ports[i] == '^' {
                splits += 1;
                if i > 0 {
                    new_beams[i - 1] = true;
                }
                if i + 1 < new_beams.len() {
                    new_beams[i + 1] = true;
                }
            } else if beam {
                new_beams[i] = true;
            }
        }
        beams = new_beams;
    }
    splits.to_string()
}

fn count_paths(
    data: &[&str],
    row: usize,
    col: usize,
    memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if row >= data.len() {
        return 1;
    }

    if let Some(&result) = memo.get(&(row, col)) {
        return result;
    }

    let chars: Vec<char> = data[row].chars().collect();

    if col >= chars.len() {
        return 0;
    }

    let result = if chars[col] == '^' {
        let mut total = 0;
        if col > 0 {
            total += count_paths(data, row + 1, col - 1, memo);
        }
        if col + 1 < chars.len() {
            total += count_paths(data, row + 1, col + 1, memo);
        }
        total
    } else {
        count_paths(data, row + 1, col, memo)
    };

    memo.insert((row, col), result);
    result
}

fn part2(input: &str) -> String {
    let data: Vec<&str> = input.trim().lines().collect();
    let start = data[0].find('S').unwrap();
    let mut memo = HashMap::new();
    count_paths(&data, 1, start, &mut memo).to_string()
}
