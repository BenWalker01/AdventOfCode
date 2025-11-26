pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> String {
    let mut total_chars = 0;
    let mut total_mem = 0;
    for line in input.trim().lines() {
        total_chars += line.len();
        total_mem += in_memory_len(line);
    }

    (total_chars - total_mem).to_string()
}
fn in_memory_len(line: &str) -> usize {
    if line.len() < 2 {
        return 0;
    }
    let s = &line[1..line.len() - 1];
    let bs = s.as_bytes();
    let mut i = 0;
    let mut count = 0;
    while i < bs.len() {
        if bs[i] == b'\\' {
            i += 1;
            if i >= bs.len() {
                break;
            }
            match bs[i] {
                b'\\' | b'"' | b'n' | b't' | b'r' => {
                    i += 1;
                    count += 1;
                }
                b'x' => {
                    i += 1;
                    if i + 1 < bs.len() {
                        i += 2;
                    } else {
                        i = bs.len();
                    }
                    count += 1;
                }
                _ => {
                    i += 1;
                    count += 1;
                }
            }
        } else {
            let ch = s[i..].chars().next().unwrap();
            i += ch.len_utf8();
            count += 1;
        }
    }
    count
}

fn part2(input: &str) -> String {
    let mut total_chars = 0;
    let mut total_mem = 0;
    for line in input.trim().lines() {
        let encoded = encode_str(line);
        total_chars += encoded.len();
        total_mem += in_memory_len(&encoded);
    }

    (total_chars - total_mem).to_string()
}

fn encode_str(line: &str) -> String {
    let mut encoded = String::new();
    let mut i = 0;
    let bs = line.as_bytes();
    while i < bs.len() {
        if bs[i] == b'"' || bs[i] == b'\\' {
            // escape quotes and backslashes
            encoded.push('\\');
            encoded.push(bs[i] as char);
            i += 1;
        } else {
            let ch = line[i..].chars().next().unwrap();
            encoded.push(ch);
            i += ch.len_utf8();
        }
    }
    format!("\"{}\"", encoded)
}
