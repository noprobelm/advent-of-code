//! # Advent of Code - 2023 Day 2: Cube Conundrum
//!
//! ## Problem Description: Part 1
//! Something is wrong with global snow production, and you've been selected to take a look. The Elves have even given
//! you a map; on it, they've used stars to mark the top fifty locations that are likely to be having problems.
//!
//! You've been doing this long enough to know that to restore snow operations, you need to check all fifty stars by
//! December 25th.
//!
//! Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second
//! puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!
//!
//! You try to ask why they can't just use a weather machine ("not powerful enough") and where they're even sending you
//! ("the sky") and why your map looks mostly blank ("you sure ask a lot of questions") and hang on did you just say the
//! sky ("of course, where do you think snow comes from") when you realize that the Elves are already loading you into a
//! trebuchet ("please hold still, we need to strap you in").
//!
//! As they're making the final adjustments, they discover that their calibration document (your puzzle input) has been
//! amended by a very young Elf who was apparently just excited to show off her art skills. Consequently, the Elves are
//! having trouble reading the values on the document.
//!
//! The newly-improved calibration document consists of lines of text; each line originally contained a specific
//! calibration value that the Elves now need to recover. On each line, the calibration value can be found by combining
//! the first digit and the last digit (in that order) to form a single two-digit number.
//!
//! For example:
//! - 1abc2
//! - pqr3stu8vwx
//! - a1b2c3d4e5f
//! - treb7uchet
//!
//! In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces
//! 142.
//!
//! Consider your entire calibration document. What is the sum of all of the calibration values?
//!
//! ## Problem Description: Part 2
//!
//! Your calculation isn't quite right. It looks like some of the digits are actually spelled out with letters: one,
//! two, three, four, five, six, seven, eight, and nine also count as valid "digits".
//!
//! Equipped with this new information, you now need to find the real first and last digit on each line. For example:
//!
//! - two1nine
//! - eightwothree
//! - abcone2threexyz
//! - xtwone3four
//! - 4nineeightseven2
//! - zoneight234
//! - 7pqrstsixteen
//!
//! In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. Adding these together produces 281.
//!
//! What is the sum of all of the calibration values?

use aoc::PuzzleInput;
use std::collections::HashMap;

fn main() {
    let p = PuzzleInput::new("../data/1.txt");
    let lines = p.lines();

    let part_1 = part_1(&lines);
    println!("Part 1: {part_1}");

    let part_2 = part_2(&lines);
    println!("Part 2: {part_2}");
}

fn part_1(lines: &Vec<&str>) -> u32 {
    let mut answer = 0;
    for line in lines {
        let digits: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();
        let first = digits.first().unwrap();
        let last = digits.last().unwrap();
        let digit = format!("{}{}", first, last);

        answer += digit.parse::<u32>().unwrap();
    }
    answer
}

fn part_2(lines: &Vec<&str>) -> u32 {
    let mut answer: u32 = 0;
    let pattern: [&str; 18] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9",
    ];

    // The max character length found among our patterns

    let max_match_len = 5;

    let mapper: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    for line in lines {
        let mut start: usize = 0;
        let mut end: usize = 0;
        let mut matches: Vec<u32> = Vec::new();
        while start < line.len() {
            if pattern.contains(&&line[start..=end]) {
                let matched = &line[start..=end];
                // We matched a word
                if let Some(val) = mapper.get(matched) {
                    matches.push(*val);
                    start = end;
                // We matched a digit
                } else {
                    matches.push(matched.parse::<u32>().expect("Invalid u32"));
                    start += 1;
                    end = start;
                }
            } else if end == line.len() - 1 || end - start == max_match_len {
                start += 1;
                end = start;
            } else {
                end += 1;
            }
        }
        let n = [*matches.first().unwrap(), *matches.last().unwrap()]
            .iter()
            .fold(0, |acc, x| acc * 10 + x);

        answer += n;
    }
    answer
}
