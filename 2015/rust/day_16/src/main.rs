//! # Advent of Code - 2015 Day 16: Aunt Sue
//!
//! ## Problem Description: Part 1
//! Your Aunt Sue has given you a wonderful gift, and you'd like to send her a thank you card. However, there's a small
//! problem: she signed it "From, Aunt Sue".
//!
//! You have 500 Aunts named "Sue".
//! So, to avoid sending the card to the wrong person, you need to figure out which Aunt Sue (which you conveniently
//! number 1 to 500, for sanity) gave you the gift. You open the present and, as luck would have it, good ol' Aunt Sue
//! got you a My First Crime Scene Analysis Machine! Just what you wanted. Or needed, as the case may be.
//! The My First Crime Scene Analysis Machine (MFCSAM for short) can detect a few specific compounds in a given sample,
//! as well as how many distinct kinds of those compounds there are. According to the instructions, these are what the
//! MFCSAM can detect:
//!
//! - children, by human DNA age analysis.
//! - cats. It doesn't differentiate individual breeds.
//! - Several seemingly random breeds of dog: samoyeds, pomeranians, akitas, and vizslas.
//! - goldfish. No other kinds of fish.
//! - trees, all in one group.
//! - cars, presumably by exhaust or gasoline or something.
//! - perfumes, which is handy, since many of your Aunts Sue wear a few kinds.
//!
//! In fact, many of your Aunts Sue have many of these. You put the wrapping from the gift into the MFCSAM. It beeps
//! inquisitively at you a few times and then prints out a message on ticker tape:
//!
//! - children: 3
//! - cats: 7
//! - samoyeds: 2
//! - pomeranians: 3
//! - akitas: 0
//! - vizslas: 0
//! - goldfish: 5
//! - trees: 3
//! - cars: 2
//! - perfumes: 1
//!
//! You make a list of the things you can remember about each Aunt Sue. Things missing from your list aren't zero - you
//! simply don't remember the value.
//!
//! What is the number of the Sue that got you the gift?
//!
//! ## Problem Description: Part 2
//! As you're about to send the thank you note, something in the MFCSAM's instructions catches your eye. Apparently, it
//! has an outdated retroencabulator, and so the output from the machine isn't exact values - some of them indicate
//! ranges.
//!
//! In particular, the cats and trees readings indicates that there are greater than that many (due to the unpredictable
//! nuclear decay of cat dander and tree pollen), while the pomeranians and goldfish readings indicate that there are
//! fewer than that many (due to the modial interaction of magnetoreluctance).
//!
//! What is the number of the real Aunt Sue?

use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

fn main() {
    let lines = lines_from_file("input.txt");
    let correct_sue: HashMap<&str, u32> = HashMap::from([
        ("id", 0),
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]);

    let part_1 = part_1(&correct_sue, &lines);
    println!("Part 1: The Aunt Sue with the greatest number of matches from the 'correct' Aunt Sue is Sue #{part_1}");

    let part_2 = part_2(&correct_sue, &lines);
    println!("Part 2: The Aunt Sue with the greatest number of matches from the 'correct' Aunt Sue is Sue #{part_2}");
}

/// Checks to see if what we remember about each of our Aunt Sues matches the information we know about the Sue who gave
/// us our gift. If so, and her 'score' is higher than previous matches, she becomes the new leading candidate. After all
/// 500 Sues have been explored, we are left with the 'correct' Sue.
fn part_1(sue: &HashMap<&str, u32>, lines: &Vec<String>) -> u32 {
    let mut leading_sue_id: u32 = 0;
    let mut top_scoring: u32 = 0;
    for line in lines {
        let mut score: u32 = 0;
        let other_sue = parse_sue(&line);
        for k in other_sue.keys().filter(|&k| k != &"id") {
            if sue.get(k) == other_sue.get(k) {
                score += 1;
            } else {
                continue;
            }
        }
        if score > top_scoring {
            top_scoring = score;
            leading_sue_id = *other_sue.get("id").unwrap()
        }
    }
    leading_sue_id
}

/// Same solution as part_1, but now we're checking for ranges among certain values instead of exact matches.
fn part_2(sue: &HashMap<&str, u32>, lines: &Vec<String>) -> u32 {
    let mut leading_sue_id: u32 = 0;
    let mut top_scoring: u32 = 0;
    for line in lines {
        let mut score: u32 = 0;
        let other_sue = parse_sue(&line);
        for k in other_sue.keys().filter(|&k| k != &"id") {
            match k {
                &"cats" | &"trees" => {
                    if other_sue.get(k) >= sue.get(k) {
                        score += 1;
                    }
                }
                &"pomeranians" | &"goldfish" => {
                    if other_sue.get(k) <= sue.get(k) {
                        score += 1;
                    }
                }
                _ => {
                    if other_sue.get(k) == sue.get(k) {
                        score += 1;
                    } else {
                        continue;
                    }
                }
            }
        }
        if score > top_scoring {
            top_scoring = score;
            leading_sue_id = *other_sue.get("id").unwrap()
        }
    }
    leading_sue_id
}

/// Parses each line of the puzzle input into a HashMap that can be checked against what we defined in fn main
fn parse_sue(s: &String) -> HashMap<&str, u32> {
    let (i, p) = s.split_once(": ").unwrap();

    let mut sue: HashMap<&str, u32> = p
        .split(", ")
        .map(|s| s.split(": ").collect::<Vec<&str>>())
        .map(|v| (v[0], v[1].parse::<u32>().expect("Invalid u32!")))
        .collect();

    sue.insert(
        "id",
        i.chars()
            .filter(|c| c.is_digit(10))
            .map(|c| c.to_digit(10).unwrap())
            .fold(0, |ans, i| ans * 10 + i),
    );

    sue
}
fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
