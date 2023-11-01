use std::fs;

fn main() {
    let s = fs::read_to_string("input.txt")
        .expect("Unable to read file")
        .replace("\n", "");
    part_1(&s);
    part_2(&s);
}

fn part_1(s: &String) {
    let floor: i32 = s
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => panic!("Invalid instruction detected. Use '(' or '('"),
        })
        .fold(0, |acc, x| acc + x);
    println!("(Part 1): Santa ended up on floor #{floor}");
}

fn part_2(s: &String) {
    let mut floor: i32 = 0;
    let mut steps = s.chars().map(|c| match c {
        '(' => 1,
        ')' => -1,
        _ => panic!("Invalid instruction detected. Use '(' or '('"),
    });
    for (i, step) in steps.enumerate() {
        floor += step;
        if floor == -1 {
            println!("(Part 2): Santa entered the basement at step #{}", i + 1);
            break;
        }
    }
}
