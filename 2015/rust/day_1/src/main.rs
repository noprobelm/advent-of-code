//! # Advent of Code - 2015 Day 1:
//!
//! Santa is trying to deliver presents in a large apartment building, but he can't find the right
//! floor - the directions he got are a little confusing. He starts on the ground floor (floor 0)
//! and then follows the instructions one character at a time.
//!
//! An opening parenthesis, (, means he should go up one floor, and a closing parenthesis, ), means
//! he should go down one floor.
//!
//! The apartment building is very tall, and the basement is very deep; he will never find the top
//! or bottom floors.
//!
//! For example:
//!
//!    (()) and ()() both result in floor 0.
//!    ((( and (()(()( both result in floor 3.
//!    ))((((( also results in floor 3.
//!    ()) and ))( both result in floor -1 (the first basement level).
//!    ))) and )())()) both result in floor -3.
//!
//! To what floor do the instructions take Santa?
//!
//! An opening parenthesis, (, means he should go up one floor, and a closing parenthesis, ), means he should go down one floor.
//!
//! The apartment building is very tall, and the basement is very deep; he will never find the top or bottom floors.
//!
//! ## Solution
//!
//!
use std::fs;

/// This is main()
pub fn main() {
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
