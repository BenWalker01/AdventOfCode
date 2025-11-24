pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> String {
    let mut nice_count = 0;

    for line in input.trim().lines() {
        let mut vowel_count = 0;
        let mut has_double = false;
        let mut has_banned = false;

        let mut prev: Option<char> = None;
        for c in line.chars() {
            if matches!(c, 'a' | 'e' | 'i' | 'o' | 'u') {
                vowel_count += 1;
            }
            if let Some(p) = prev {
                if p == c {
                    has_double = true;
                }
                if (p == 'a' && c == 'b')
                    || (p == 'c' && c == 'd')
                    || (p == 'p' && c == 'q')
                    || (p == 'x' && c == 'y')
                {
                    has_banned = true;
                }
            }

            prev = Some(c);
        }

        if vowel_count >= 3 && has_double && !has_banned {
            nice_count += 1;
        }
    }

    nice_count.to_string()
}

fn part2(input: &str) -> String {
    let mut nice_count = 0;

    for line in input.trim().lines() {
        let mut has_pair = false;
        let mut has_repeat = false;
        let mut pairs: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();

        let len = line.len();
        if len < 2 {
            continue;
        }
        let bytes = line.as_bytes();
        for i in 0..len {
            if i + 2 < len && bytes[i] == bytes[i + 2] {
                has_repeat = true;
            }
            if i + 1 < len {
                let pair = &line[i..i + 2];
                if let Some(&prev_idx) = pairs.get(pair) {
                    if i - prev_idx > 1 {
                        has_pair = true;
                    }
                } else {
                    pairs.insert(pair, i);
                }
            }
        }
        if has_pair && has_repeat {
            nice_count += 1;
        }
    }
    nice_count.to_string()
}
