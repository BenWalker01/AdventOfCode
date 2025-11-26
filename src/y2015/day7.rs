use std::collections::HashMap;

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[derive(Clone, Debug)]
enum Instruction {
    Value(u16),
    Wire(String),
    Not(String),
    And(String, String),
    Or(String, String),
    LShift(String, u32),
    RShift(String, u32),
}

fn part1(input: &str) -> String {
    let mut instructions: HashMap<String, Instruction> = HashMap::new();
    let mut cache: HashMap<String, u16> = HashMap::new();

    // Parse all instructions
    for line in input.trim().lines() {
        let (op, result) = line
            .split_once(" -> ")
            .map(|(l, r)| (l.trim(), r.trim()))
            .expect("invalid instruction format");

        let instruction = parse_instruction(op);
        instructions.insert(result.to_string(), instruction);
    }

    // Get value for wire "a"
    let value = compute_wire(&instructions, &mut cache, "a");
    value.to_string()
}

fn parse_instruction(op: &str) -> Instruction {
    let tokens: Vec<&str> = op.split_ascii_whitespace().collect();

    match tokens.as_slice() {
        [val] => {
            if let Ok(num) = val.parse::<u16>() {
                Instruction::Value(num)
            } else {
                Instruction::Wire(val.to_string())
            }
        }
        ["NOT", reg] => Instruction::Not(reg.to_string()),
        [reg1, "AND", reg2] => Instruction::And(reg1.to_string(), reg2.to_string()),
        [reg1, "OR", reg2] => Instruction::Or(reg1.to_string(), reg2.to_string()),
        [reg1, "LSHIFT", shift] => {
            Instruction::LShift(reg1.to_string(), shift.parse::<u32>().unwrap_or(0))
        }
        [reg1, "RSHIFT", shift] => {
            Instruction::RShift(reg1.to_string(), shift.parse::<u32>().unwrap_or(0))
        }
        _ => Instruction::Value(0),
    }
}

fn compute_wire(
    instructions: &HashMap<String, Instruction>,
    cache: &mut HashMap<String, u16>,
    wire: &str,
) -> u16 {
    if let Some(&value) = cache.get(wire) {
        return value;
    }

    if let Ok(num) = wire.parse::<u16>() {
        return num;
    }

    let instruction = instructions
        .get(wire)
        .cloned()
        .unwrap_or(Instruction::Value(0));

    let value = match instruction {
        Instruction::Value(v) => v,
        Instruction::Wire(w) => compute_wire(instructions, cache, &w),
        Instruction::Not(w) => !compute_wire(instructions, cache, &w),
        Instruction::And(w1, w2) => {
            let v1 = compute_wire(instructions, cache, &w1);
            let v2 = compute_wire(instructions, cache, &w2);
            v1 & v2
        }
        Instruction::Or(w1, w2) => {
            let v1 = compute_wire(instructions, cache, &w1);
            let v2 = compute_wire(instructions, cache, &w2);
            v1 | v2
        }
        Instruction::LShift(w, shift) => {
            let v = compute_wire(instructions, cache, &w);
            v << shift
        }
        Instruction::RShift(w, shift) => {
            let v = compute_wire(instructions, cache, &w);
            v >> shift
        }
    };

    cache.insert(wire.to_string(), value);
    value
}

fn part2(input: &str) -> String {
    // First, get the value of wire "a" from part1
    let mut instructions: HashMap<String, Instruction> = HashMap::new();
    let mut cache: HashMap<String, u16> = HashMap::new();

    for line in input.trim().lines() {
        let (op, result) = line
            .split_once(" -> ")
            .map(|(l, r)| (l.trim(), r.trim()))
            .expect("invalid instruction format");

        let instruction = parse_instruction(op);
        instructions.insert(result.to_string(), instruction);
    }

    let a_value_part1 = compute_wire(&instructions, &mut cache, "a");

    let mut instructions: HashMap<String, Instruction> = HashMap::new();
    let mut cache: HashMap<String, u16> = HashMap::new();

    for line in input.trim().lines() {
        let (op, result) = line
            .split_once(" -> ")
            .map(|(l, r)| (l.trim(), r.trim()))
            .expect("invalid instruction format");

        if result == "b" {
            instructions.insert(result.to_string(), Instruction::Value(a_value_part1));
        } else {
            let instruction = parse_instruction(op);
            instructions.insert(result.to_string(), instruction);
        }
    }

    let a_value_part2 = compute_wire(&instructions, &mut cache, "a");
    a_value_part2.to_string()
}
