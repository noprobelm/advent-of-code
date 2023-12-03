use aoc::PuzzleInput;
use std::collections::HashMap;

fn main() {
    let p = PuzzleInput::new("../data/7.txt");
    let lines = p.lines();

    let part_1 = part_1(&lines);
    println!("Part 1: {part_1}");
    let part_2 = part_2(part_1, &lines);
    println!("Part 2: {part_2}")
}

fn part_1(lines: &Vec<&str>) -> u32 {
    let mut mapper: HashMap<&str, u32> = HashMap::new();
    let gates: Vec<&str> = vec!["AND", "OR", "NOT", "LSHIFT", "RSHIFT"];
    let num_gates = lines.len().try_into().unwrap();

    while mapper.len() < num_gates {
        for line in lines {
            let mut split: Vec<&str> = line.split(" ").filter(|&s| s != "->").collect();
            let mut input_wires: Vec<u32> = Vec::with_capacity(2);
            let output_wire = split.pop().unwrap();

            if let Some(_result) = mapper.get(output_wire) {
                continue;
            }

            for (_i, s) in split.clone().iter().enumerate() {
                if let Ok(val) = s.parse::<u32>() {
                    input_wires.push(val);
                } else if let Some(val) = mapper.get(s) {
                    input_wires.push(*val);
                }
            }

            if input_wires.len() == split.iter().filter(|x| !gates.contains(x)).count() {
                if let Some(gate_id) = split.iter().find(|x| gates.contains(x)) {
                    let gate = match *gate_id {
                        "NOT" => LogicGate::NOT(input_wires[0]),
                        "AND" => LogicGate::AND(input_wires[0], input_wires[1]),
                        "OR" => LogicGate::OR(input_wires[0], input_wires[1]),
                        "LSHIFT" => LogicGate::LSHIFT(input_wires[0], input_wires[1]),
                        "RSHIFT" => LogicGate::RSHIFT(input_wires[0], input_wires[1]),
                        _ => panic!["Expected logic gate"],
                    };
                    mapper.insert(output_wire, gate.operate());
                } else {
                    mapper.insert(output_wire, input_wires[0]);
                }
            }
        }
    }
    *mapper.get(&"a").unwrap()
}

fn part_2(a_val: u32, lines: &Vec<&str>) -> u32 {
    let mut mapper: HashMap<&str, u32> = HashMap::new();
    mapper.insert("b", a_val);
    let gates: Vec<&str> = vec!["AND", "OR", "NOT", "LSHIFT", "RSHIFT"];
    let num_gates = lines.len().try_into().unwrap();

    while mapper.len() < num_gates {
        for line in lines {
            let mut split: Vec<&str> = line.split(" ").filter(|&s| s != "->").collect();
            let mut input_wires: Vec<u32> = Vec::with_capacity(2);
            let output_wire = split.pop().unwrap();

            if let Some(_result) = mapper.get(output_wire) {
                continue;
            }

            for (_i, s) in split.clone().iter().enumerate() {
                if let Ok(val) = s.parse::<u32>() {
                    input_wires.push(val);
                } else if let Some(val) = mapper.get(s) {
                    input_wires.push(*val);
                }
            }

            if input_wires.len() == split.iter().filter(|x| !gates.contains(x)).count() {
                if let Some(gate_id) = split.iter().find(|x| gates.contains(x)) {
                    let gate = match *gate_id {
                        "NOT" => LogicGate::NOT(input_wires[0]),
                        "AND" => LogicGate::AND(input_wires[0], input_wires[1]),
                        "OR" => LogicGate::OR(input_wires[0], input_wires[1]),
                        "LSHIFT" => LogicGate::LSHIFT(input_wires[0], input_wires[1]),
                        "RSHIFT" => LogicGate::RSHIFT(input_wires[0], input_wires[1]),
                        _ => panic!["Expected logic gate"],
                    };
                    mapper.insert(output_wire, gate.operate());
                } else {
                    mapper.insert(output_wire, input_wires[0]);
                }
            }
        }
    }

    *mapper.get("a").unwrap()
}

enum LogicGate {
    NOT(u32),
    AND(u32, u32),
    OR(u32, u32),
    LSHIFT(u32, u32),
    RSHIFT(u32, u32),
}

impl LogicGate {
    fn operate(&self) -> u32 {
        match self {
            LogicGate::NOT(a) => !a,
            LogicGate::AND(a, b) => a & b,
            LogicGate::OR(a, b) => a | b,
            LogicGate::LSHIFT(a, b) => a << b,
            LogicGate::RSHIFT(a, b) => a >> b,
        }
    }
}
