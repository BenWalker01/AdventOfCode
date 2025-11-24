use hex_literal::hex;
use md5;
use rayon::prelude::*;

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> String {
    for a in 0..700000 {
        let s = format!("{}{}", input, a);
        let digest = md5::compute(s);
        let hex = format!("{:x}", digest);
        if hex.starts_with("00000") {
            return a.to_string();
        }
    }

    "0".to_string()
}

fn part2(input: &str) -> String {
    let limit = 1_000_000_000usize;
    (0..limit)
        .into_par_iter()
        .find_first(|&a| {
            let s = format!("{}{}", input, a);
            let digest = md5::compute(s);
            // 6 hex zeroes == first 3 bytes are zero
            digest[0] == 0 && digest[1] == 0 && digest[2] == 0
        })
        .map(|a| a.to_string())
        .unwrap_or_else(|| "0".to_string())
}
