use itertools::Itertools;

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> String {
    let mut people: Vec<&str> = input
        .trim()
        .lines()
        .map(|line| line.split_whitespace().next().unwrap())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    people.sort_unstable();
    let number_of_people = people.len();

    let mut happiness: std::collections::HashMap<&str, std::collections::HashMap<&str, i32>> =
        std::collections::HashMap::new();

    for line in input.trim().lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let person = parts[0];
        let gain = parts[2] == "gain";
        let units: i32 = parts[3].parse().unwrap();
        let neighbor = parts[10].trim_end_matches('.');
        happiness
            .entry(person)
            .or_default()
            .insert(neighbor, if gain { units } else { -units });
    }

    let mut best_happy = i32::MIN;

    for perm in people.into_iter().permutations(number_of_people) {
        let mut sum = 0;
        for i in 0..number_of_people {
            let p = perm[i];
            let left = (i + number_of_people - 1) % number_of_people;
            let right = (i + 1) % number_of_people;
            let left_person = perm[left];
            let right_person = perm[right];

            sum += *happiness
                .get(p)
                .and_then(|m| m.get(left_person))
                .unwrap_or(&0);
            sum += *happiness
                .get(p)
                .and_then(|m| m.get(right_person))
                .unwrap_or(&0);
        }
        if sum > best_happy {
            best_happy = sum;
        }
    }

    best_happy.to_string()
}

fn part2(input: &str) -> String {
    let mut people: Vec<&str> = input
        .trim()
        .lines()
        .map(|line| line.split_whitespace().next().unwrap())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    people.sort_unstable();

    let mut happiness: std::collections::HashMap<&str, std::collections::HashMap<&str, i32>> =
        std::collections::HashMap::new();

    for line in input.trim().lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let person = parts[0];
        let gain = parts[2] == "gain";
        let units: i32 = parts[3].parse().unwrap();
        let neighbor = parts[10].trim_end_matches('.');
        happiness
            .entry(person)
            .or_default()
            .insert(neighbor, if gain { units } else { -units });
    }

    people.push("Me");

    for person in &people {
        let p = *person;
        if p == "Me" {
            continue;
        }
        happiness.entry(p).or_default().insert("Me", 0);
    }

    let mut me_map: std::collections::HashMap<&str, i32> = std::collections::HashMap::new();
    for person in &people {
        let p = *person;
        if p == "Me" {
            continue;
        }
        me_map.insert(p, 0);
    }
    happiness.insert("Me", me_map);

    let number_of_people = people.len();
    let mut best_happy = i32::MIN;

    for perm in people.into_iter().permutations(number_of_people) {
        let mut sum = 0;
        for i in 0..number_of_people {
            let p = perm[i];
            let left = (i + number_of_people - 1) % number_of_people;
            let right = (i + 1) % number_of_people;
            let left_person = perm[left];
            let right_person = perm[right];

            sum += *happiness
                .get(p)
                .and_then(|m| m.get(left_person))
                .unwrap_or(&0);
            sum += *happiness
                .get(p)
                .and_then(|m| m.get(right_person))
                .unwrap_or(&0);
        }
        if sum > best_happy {
            best_happy = sum;
        }
    }

    best_happy.to_string()
}
