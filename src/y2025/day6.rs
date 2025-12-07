pub fn solve(input: &str) {
    //println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> String {
    let mut sum = 0;
    let data: Vec<&str> = input.trim().lines().collect();
    let data: Vec<Vec<&str>> = data
        .iter()
        .map(|line| line.split_whitespace().collect())
        .collect();

    for i in 0..data[0].len() {
        let a = data[0][i].parse::<u64>().unwrap();
        let b = data[1][i].parse::<u64>().unwrap();
        let c = data[2][i].parse::<u64>().unwrap();
        let d = data[3][i].parse::<u64>().unwrap();
        let opr = data[4][i];
        sum += match opr {
            "+" => a + b + c + d,
            "*" => a * b * c * d,
            _ => 0,
        };
    }

    sum.to_string()
}

fn part2(input: &str) -> String {
    let mut sum = 0;
    let data: Vec<&str> = input.trim().lines().collect();
    let data: Vec<Vec<char>> = data.iter().map(|line| line.chars().collect()).collect();

    let mut numbers: Vec<u64> = Vec::new();
    let mut current_opr = "+";
    let max_len = data.iter().map(|row| row.len()).min().unwrap_or(0);
    for col in 0..max_len {
        let a = data[0][col];
        let b = data[1][col];
        let c = data[2][col];
        let d = data[3][col];
        let opr = data[4][col];
        if opr == '*' || opr == '+' {
            println!("{:?}", numbers);
            sum += if current_opr == "+" {
                numbers.iter().sum::<u64>()
            } else {
                numbers.iter().product::<u64>()
            };

            current_opr = if opr == '+' { "+" } else { "*" };
            numbers.clear();
        }
        let num = format!("{}{}{}{}", a, b, c, d);
        let parsed = num.trim().parse::<u64>();
        if let Ok(n) = parsed {
            numbers.push(n);
        } else {
            println!("Failed to parse: {}", num);
        }
    }

    sum += if current_opr == "+" {
        numbers.iter().sum::<u64>()
    } else {
        numbers.iter().product::<u64>()
    };

    sum.to_string()
}
