use std::collections::{HashSet, VecDeque};
use z3::ast::Ast;
use z3::{Config, Context, Optimize};

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn press_button(state: &mut [String], button: &[u32]) {
    for &c in button {
        let idx = c as usize;
        state[idx] = if state[idx] == "." {
            "#".to_string()
        } else {
            ".".to_string()
        };
    }
}

fn find_state(buttons: &[Vec<u32>], initial_state: &[&str], target: &[&str]) -> u32 {
    let initial: Vec<String> = initial_state.iter().map(|s| s.to_string()).collect();
    let target_vec: Vec<String> = target.iter().map(|s| s.to_string()).collect();

    if initial == target_vec {
        return 0;
    }
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((initial.clone(), 0u32));
    visited.insert(initial);
    while let Some((state, presses)) = queue.pop_front() {
        for button in buttons {
            let mut new_state = state.clone();
            press_button(&mut new_state, button);
            if new_state == target_vec {
                return presses + 1;
            }
            if visited.insert(new_state.clone()) {
                queue.push_back((new_state, presses + 1));
            }
        }
    }

    u32::MAX
}

fn part1(input: &str) -> String {
    let mut total = 0;
    for machine in input.trim().lines() {
        let data: Vec<&str> = machine.split_ascii_whitespace().collect();
        let target: Vec<&str> = data[0].split("").collect::<Vec<&str>>()[2..data[0].len()].to_vec();
        let state: Vec<&str> = vec!["."; target.len()];
        let buttons: Vec<Vec<u32>> = data[1..data.len() - 1]
            .iter()
            .map(|b| b.chars().filter_map(|c| c.to_digit(10)).collect())
            .collect();
        total += find_state(&buttons, &state, &target);
    }
    total.to_string()
}

fn part2(input: &str) -> String {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let mut total = 0u64;

    for machine in input.trim().lines() {
        let optimizer = Optimize::new(&ctx);
        let data: Vec<&str> = machine.split_ascii_whitespace().collect();
        let num_buttons = data.len() - 2;
        let buttons: Vec<Vec<usize>> = data[1..num_buttons + 1]
            .iter()
            .map(|b| {
                b.trim_matches(|c| c == '(' || c == ')')
                    .split(',')
                    .filter_map(|s| s.trim().parse().ok())
                    .collect()
            })
            .collect();

        let target: Vec<u32> = data[data.len() - 1]
            .trim_matches(|c| c == '{' || c == '}')
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();

        let button_vars: Vec<_> = (0..num_buttons)
            .map(|i| z3::ast::Int::new_const(&ctx, format!("b{}", i)))
            .collect();

        for var in &button_vars {
            optimizer.assert(&var.ge(&z3::ast::Int::from_i64(&ctx, 0)));
        }

        let num_bits = target.len();
        let final_state: Vec<_> = (0..num_bits)
            .map(|bit_idx| {
                let mut expr = z3::ast::Int::from_i64(&ctx, 0);

                for (button_idx, button) in buttons.iter().enumerate() {
                    if button.contains(&bit_idx) {
                        expr = z3::ast::Int::add(&ctx, &[&expr, &button_vars[button_idx]]);
                    }
                }
                expr
            })
            .collect();

        for (i, final_bit) in final_state.iter().enumerate() {
            optimizer.assert(&final_bit._eq(&z3::ast::Int::from_i64(&ctx, target[i] as i64)));
        }

        let total_presses = z3::ast::Int::add(&ctx, &button_vars.iter().collect::<Vec<_>>());
        optimizer.minimize(&total_presses);

        if optimizer.check(&[]) == z3::SatResult::Sat {
            if let Some(model) = optimizer.get_model() {
                let sum: u64 = button_vars
                    .iter()
                    .map(|var| {
                        if let Some(eval_result) = model.eval(var, true) {
                            eval_result.as_i64().unwrap_or(0) as u64
                        } else {
                            0
                        }
                    })
                    .sum();
                total += sum;
            }
        }
    }

    total.to_string()
}
