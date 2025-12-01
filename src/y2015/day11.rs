pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn is_valid(password: &str) -> bool {
    let b = password.as_bytes();

    if b.iter().any(|&c| c == b'i' || c == b'o' || c == b'l') {
        return false;
    }

    let mut has_straight = false;
    for w in b.windows(3) {
        if w[0] + 1 == w[1] && w[1] + 1 == w[2] {
            has_straight = true;
            break;
        }
    }
    if !has_straight {
        return false;
    }

    let mut pairs = Vec::new();
    let mut i = 0;
    while i + 1 < b.len() {
        if b[i] == b[i + 1] {
            pairs.push(b[i]);
            i += 2;
        } else {
            i += 1;
        }
    }
    pairs.sort_unstable();
    pairs.dedup();
    pairs.len() >= 2
}

fn generate_new_password(input: &str) -> &str {
    let b = input.as_bytes();
    let mut v = b.to_vec();
    let mut i = v.len();
    while i > 0 {
        i -= 1;
        if v[i] == b'z' {
            v[i] = b'a';
        } else {
            v[i] += 1;
            break;
        }
    }
    let s = String::from_utf8(v).unwrap();
    Box::leak(s.into_boxed_str())
}

fn part1(input: &str) -> String {
    let mut new_password = generate_new_password(input);
    while !is_valid(new_password) {
        new_password = generate_new_password(new_password);
    }
    new_password.to_string()
}

fn part2(input: &str) -> String {
    let mut new_password = generate_new_password(input);
    while !is_valid(new_password) {
        new_password = generate_new_password(new_password);
    }
    new_password = generate_new_password(new_password);
    while !is_valid(new_password) {
        new_password = generate_new_password(new_password);
    }
    new_password.to_string()
}
