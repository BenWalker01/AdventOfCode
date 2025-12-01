use serde_json;

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> String {
    let mut total: i64 = 0;
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if (bytes[i] == b'-' && i + 1 < bytes.len() && bytes[i + 1].is_ascii_digit())
            || bytes[i].is_ascii_digit()
        {
            let start = i;
            if bytes[i] == b'-' {
                i += 1;
            }
            while i < bytes.len() && bytes[i].is_ascii_digit() {
                i += 1;
            }
            let s = std::str::from_utf8(&bytes[start..i]).unwrap();
            if let Ok(n) = s.parse::<i64>() {
                total += n;
            }
        } else {
            i += 1;
        }
    }
    total.to_string()
}

fn part2(input: &str) -> String {
    let v: serde_json::Value = serde_json::from_str(input).unwrap();
    fn sum(v: &serde_json::Value) -> i64 {
        match v {
            serde_json::Value::Number(n) => n
                .as_i64()
                .or_else(|| n.as_f64().map(|f| f as i64))
                .unwrap_or(0),
            serde_json::Value::Array(arr) => arr.iter().map(sum).sum(),
            serde_json::Value::Object(map) => {
                if map
                    .values()
                    .any(|val| matches!(val, serde_json::Value::String(s) if s == "red"))
                {
                    0
                } else {
                    map.values().map(sum).sum()
                }
            }
            _ => 0,
        }
    }
    sum(&v).to_string()
}
