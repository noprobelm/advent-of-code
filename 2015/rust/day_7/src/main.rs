use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

fn main() {
    let lines = lines_from_file("input.txt");
    part_1(&lines);
}

fn part_1(lines: &Vec<String>) {
    let mut mapper: HashMap<&str, u32> = HashMap::new();
    let mut discovered_all = false;
    let gates: Vec<&str> = vec!["AND", "OR", "NOT", "LSHIFT", "RSHIFT"];
    let mut discovered: u32 = 0;

    while discovered < lines.len().try_into().unwrap() {
        for line in lines {
            let mut wires: Vec<u32> = Vec::with_capacity(2);
            let mut split: Vec<&str> = line.split(" ").filter(|&s| s != "->").collect();
            let output_wire = split.pop().unwrap();
            if let Some(result) = mapper.get(output_wire) {
                discovered += 1;
                continue;
            }
            for (i, s) in split.clone().iter().enumerate() {
                if let Ok(val) = s.parse::<u32>() {
                    wires.push(val);
                } else if let Some(val) = mapper.get(s) {
                    wires.push(*val);
                }
            }

            if wires.len() == split.iter().filter(|x| !gates.contains(x)).count() {
                if let Some(gate_id) = split.iter().find(|x| gates.contains(x)) {
                    let gate = match *gate_id {
                        "NOT" => LogicGate::NOT(wires[0]),
                        "AND" => LogicGate::AND(wires[0], wires[1]),
                        "OR" => LogicGate::OR(wires[0], wires[1]),
                        "LSHIFT" => LogicGate::LSHIFT(wires[0], wires[1]),
                        "RSHIFT" => LogicGate::RSHIFT(wires[0], wires[1]),
                        _ => panic!["Expected logic gate"],
                    };
                    mapper.insert(output_wire, gate.operate());
                } else {
                    mapper.insert(output_wire, wires[0]);
                }
            }
        }
        if discovered < lines.len().try_into().unwrap() {
            discovered = 0;
        }
    }
    println!("{:?}", mapper.get("a"));
}

fn part_2(lines: &Vec<String>) {}

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
