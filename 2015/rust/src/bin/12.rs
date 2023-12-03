use serde_json::Value;
use std::fs::File;
use std::io::Read;
use std::io::Result;

fn main() {
    let json_raw = file_to_string("../data/12.txt").expect("Not a valid file");
    let json: Value = serde_json::from_str(&json_raw).expect("JSON was not well-formatted");

    let part_1 = part_1(&json);
    println!("Part 1: {part_1}");

    let part_2 = part_2(&json);
    println!("Part 2: {part_2}");
}

fn part_1(json: &Value) -> i64 {
    match json {
        Value::Number(_) => json.as_i64().unwrap(),
        Value::Array(_) => json
            .as_array()
            .unwrap()
            .iter()
            .fold(0, |acc, v| acc + part_1(v)),
        Value::Object(_) => json
            .as_object()
            .unwrap()
            .iter()
            .fold(0, |acc, (_, v)| acc + part_1(v)),
        _ => 0,
    }
}

fn part_2(json: &Value) -> i64 {
    match json {
        Value::Number(_) => json.as_i64().unwrap(),
        Value::Array(_) => json
            .as_array()
            .unwrap()
            .iter()
            .fold(0, |acc, v| acc + part_2(v)),
        Value::Object(_) => {
            let obj = json.as_object().unwrap();
            for v in obj.values() {
                if let Some(s) = v.as_str() {
                    if s == "red" {
                        return 0;
                    }
                }
            }

            obj.values().fold(0, |acc, v| acc + part_2(v))
        }
        _ => 0,
    }
}

fn file_to_string(file_path: &str) -> Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
