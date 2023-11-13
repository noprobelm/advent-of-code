use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

fn main() {
    let lines = lines_from_file("input.txt");
    let wiremap = part_1(&lines);
    println!("After connecting all wires to their respective gates, the output of wire 'a' is assessed to be {:?}", wiremap.get("a").unwrap());

    let part_2 = part_2(*wiremap.get("a").unwrap(), &lines);
    println!("{:?}", part_2)
}

fn part_1(lines: &Vec<String>) -> HashMap<&str, u32> {
    let mut mapper: HashMap<&str, u32> = HashMap::new();
    let gates: Vec<&str> = vec!["AND", "OR", "NOT", "LSHIFT", "RSHIFT"];
    let num_gates = lines.len().try_into().unwrap();

    while mapper.len() < num_gates {
        for line in lines {
            let mut split: Vec<&str> = line.split(" ").filter(|&s| s != "->").collect();
            let mut input_wires: Vec<u32> = Vec::with_capacity(2);
            let output_wire = split.pop().unwrap();

            if let Some(result) = mapper.get(output_wire) {
                continue;
            }

            for (i, s) in split.clone().iter().enumerate() {
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
    mapper
}

fn part_2(a_val: u32, lines: &Vec<String>) -> u32 {
    let mut mapper: HashMap<&str, u32> = HashMap::new();
    mapper.insert("b", a_val);
    let gates: Vec<&str> = vec!["AND", "OR", "NOT", "LSHIFT", "RSHIFT"];
    let num_gates = lines.len().try_into().unwrap();

    while mapper.len() < num_gates {
        for line in lines {
            let mut split: Vec<&str> = line.split(" ").filter(|&s| s != "->").collect();
            let mut input_wires: Vec<u32> = Vec::with_capacity(2);
            let output_wire = split.pop().unwrap();

            if let Some(result) = mapper.get(output_wire) {
                continue;
            }

            for (i, s) in split.clone().iter().enumerate() {
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

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
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
