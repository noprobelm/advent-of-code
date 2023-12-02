//! # Advent of Code - 2015 Day 1: Not Quite Lisp
//!
//! ## Problem Description: Part 1
//! Santa was hoping for a white Christmas, but his weather machine's "snow" function is powered by stars, and he's fresh
//! out! To save Christmas, he needs you to collect fifty stars by December 25th.
//!
//! Collect stars by helping Santa solve puzzles. Two puzzles will be made available on each day in the Advent calendar;
//! the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!
//!
//! Here's an easy puzzle to warm you up.
//!
//! Santa is trying to deliver presents in a large apartment building, but he can't find the right floor - the directions
//! he got are a little confusing. He starts on the ground floor (floor 0) and then follows the instructions one
//! character at a time.
//!
//! An opening parenthesis, (, means he should go up one floor, and a closing parenthesis, ), means he should go down one
//! floor.
//!
//! The apartment building is very tall, and the basement is very deep; he will never find the top or bottom floors.
//!
//! For example:
//! - (()) and ()() both result in floor 0.
//! - ((( and (()(()( both result in floor 3.
//! - ))((((( also results in floor 3.
//! - ()) and ))( both result in floor -1 (the first basement level).
//! - ))) and )())()) both result in floor -3.
//!
//! To what floor do the instructions take Santa?
//!
//! ## Problem Description: Part 2
//! Now, given the same instructions, find the position of the first character that causes him to enter the basement (floor -1). The first character in the instructions has position 1, the second character has position 2, and so on.
//!
//! For example:
//!
//! - ) causes him to enter the basement at character position 1.
//! - ()()) causes him to enter the basement at character position 5.
//!
//! What is the position of the character that causes Santa to first enter the basement?

extern crate aoc;

pub fn main() {
    let puzzle_input = aoc::PuzzleInput::new("../data/1.txt");
    let s = puzzle_input.string();

    let part_1 = part_1(s);
    println!("Part 1: {part_1}");

    let part_2 = part_2(s);
    println!("Part 2: part_2")
}

fn part_1(s: &str) -> i32 {
    s.chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        })
        .fold(0, |acc, x| acc + x)
}

fn part_2(s: &str) -> usize {
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
    println!("Santa never entered the basement. Something must be wrong with the puzzle input.");
    0
}
