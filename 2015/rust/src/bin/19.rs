//! # Advent of Code - 2015 Day 19: Medicine for Rudolph
//!
//! ## Problem Description: Part 1
//! Rudolph the Red-Nosed Reindeer is sick! His nose isn't shining very brightly, and he needs medicine.
//!
//! Red-Nosed Reindeer biology isn't similar to regular reindeer biology; Rudolph is going to need custom-made medicine. Unfortunately, Red-Nosed Reindeer chemistry isn't similar to regular reindeer chemistry, either.
//!
//! The North Pole is equipped with a Red-Nosed Reindeer nuclear fusion/fission plant, capable of constructing any Red-Nosed Reindeer molecule you need. It works by starting with some input molecule and then doing a series of replacements, one per step, until it has the right molecule.
//!
//! However, the machine has to be calibrated before it can be used. Calibration involves determining the number of molecules that can be generated in one step from a given starting point.
//!
//! For example, imagine a simpler machine that supports only the following replacements:
//!
//! H => HO
//! H => OH
//! O => HH
//!
//! Given the replacements above and starting with HOH, the following molecules could be generated:
//!
//!     HOOH (via H => HO on the first H).
//!     HOHO (via H => HO on the second H).
//!     OHOH (via H => OH on the first H).
//!     HOOH (via H => OH on the second H).
//!     HHHH (via O => HH).
//!
//! So, in the example above, there are 4 distinct molecules (not five, because HOOH appears twice) after one replacement from HOH. Santa's favorite molecule, HOHOHO, can become 7 distinct molecules (over nine replacements: six from H, and three from O).
//!
//! The machine replaces without regard for the surrounding characters. For example, given the string H2O, the transition H => OO would result in OO2O.
//!
//! Your puzzle input describes all of the possible replacements and, at the bottom, the medicine molecule for which you need to calibrate the machine. How many distinct molecules can be created after all the different ways you can do one replacement on the medicine molecule?
//! ## Problem Description: Part 2
//! Now that the machine is calibrated, you're ready to begin molecule fabrication.
//!
//! Molecule fabrication always begins with just a single electron, e, and applying replacements one at a time, just like the ones during calibration.
//!
//! For example, suppose you have the following replacements:
//!
//! e => H
//! e => O
//! H => HO
//! H => OH
//! O => HH
//!
//! If you'd like to make HOH, you start with e, and then make the following replacements:
//!
//!     e => O to get O
//!     O => HH to get HH
//!     H => OH (on the second H) to get HOH
//!
//! So, you could make HOH after 3 steps. Santa's favorite molecule, HOHOHO, can be made in 6 steps.
//!
//! How long will it take to make the medicine? Given the available replacements and the medicine molecule in your puzzle input, what is the fewest number of steps to go from e to the medicine molecule?

use aoc::PuzzleInput;
use std::collections::{HashMap, HashSet};
use regex;

fn main() {
    let p = PuzzleInput::new("../data/19.txt");
    let lines = PuzzleInput::lines(&p);
    let mut replacements: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in &lines {
        if line.contains("=>") {
            let split: Vec<&str> = line.split("=>").map(|s| s.trim()).collect();
            replacements.entry(split[0]).and_modify(|v| v.push(split[1])).or_insert(vec![split[1]]);
        }
    }
    let part_1 = part_1(lines.last().unwrap(), &replacements);
    println!("{:?}", part_1.len());
}

fn part_1(medicine: &str, replacements: &HashMap<&str, Vec<&str>>) -> HashSet<String> {
    let mut molecules: HashSet<String> = HashSet::new();
    let re = regex::Regex::new(r"[A-Z][a-z]?").unwrap();
    let matches = re.find_iter(medicine);
    for (_, mat) in matches.enumerate() {
        if let Some(replacement) = replacements.get(mat.as_str()) {
            for mol in replacement {
                let mut new_molecule: String = String::from(&medicine[0..mat.start()]);
                new_molecule.push_str(mol);
                new_molecule.push_str(&medicine[mat.end()..]);
                molecules.insert(new_molecule);
            }
        }
    }
    molecules
}

fn part_2(lines: &Vec<&str>) -> String {
    todo!();
}
