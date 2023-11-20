//! # Advent of Code - 2015 Day 1
//!
//! ## Problem Description: Part 1
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
//!    - (()) and ()() both result in floor 0.
//!    - ((( and (()(()( both result in floor 3.
//!    - ))((((( also results in floor 3.
//!    - ()) and ))( both result in floor -1 (the first basement level).
//!    - ))) and )())()) both result in floor -3.
//!
//! To what floor do the instructions take Santa?
//!
//! An opening parenthesis, (, means he should go up one floor, and a closing parenthesis, ), means
//! he should go down one floor.
//!
//! The apartment building is very tall, and the basement is very deep; he will never find the top
//! or bottom floors.
//!
//! ## Problem Description: Part 2
//! Now, given the same instructions, find the position of the first character that causes him to
//! enter the basement (floor -1). The first character in the instructions has position 1, the
//! second character has position 2, and so on.
//!
//! For example:
//!    - ) causes him to enter the basement at character position 1.
//!    -()()) causes him to enter the basement at character position 5.
//!
//! What is the position of the character that causes Santa to first enter the basement?
use std::fs;

pub fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");

    let part_1 = part_1(&input);
    println!("(Part 1): Santa ended up on floor #{part_1}");

    let part_2 = part_2(&input);
    println!("(Part 2): Santa first entered the basement at character position {part_2} of the instructions")
}

/// Iterate through each character of the puzzle input, mapping `(` and `)` to `1` or `-1`
/// (respectively). Fold the resulting iterator into a cumulative sum result.
fn part_1(s: &String) -> i32 {
    s.chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        })
        .fold(0, |acc, x| acc + x)
}

/// Map the instructions to i32 as we did in part_1. Do not fold. Iterate through each element of
/// our map and increment `floor` by the value. After we've reached `-1`, return the `i + 1` as the
/// position of the character at which the basement was first entered.
fn part_2(s: &String) -> usize {
    let mut floor: i32 = 0;
    let steps = s.chars().map(|c| match c {
        '(' => 1,
        ')' => -1,
        _ => 0,
    });
    for (i, step) in steps.enumerate() {
        floor += step;
        if floor == -1 {
            return i + 1;
        }
    }
    panic!["Santa never entered the basement. Something must be wrong with the puzzle input."];
}
