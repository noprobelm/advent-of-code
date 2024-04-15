//! # Advent of Code - 2015 Day 20: Infinite Elves and Infinite Houses
//!
//! ## Problem Description: Part 1
//! To keep the Elves busy, Santa has them deliver some presents by hand, door-to-door. He sends them down a street with infinite houses numbered sequentially: 1, 2, 3, 4, 5, and so on.
//!
//! Each Elf is assigned a number, too, and delivers presents to houses based on that number:
//!
//! - The first Elf (number 1) delivers presents to every house: 1, 2, 3, 4, 5, ....
//! - The second Elf (number 2) delivers presents to every second house: 2, 4, 6, 8, 10, ....
//! - Elf number 3 delivers presents to every third house: 3, 6, 9, 12, 15, ....
//!
//! There are infinitely many Elves, numbered starting with 1. Each Elf delivers presents equal to ten times his or her number at each house.
//!
//! So, the first nine houses on the street end up like this:
//!
//! - House 1 got 10 presents.
//! - House 2 got 30 presents.
//! - House 3 got 40 presents.
//! - House 4 got 70 presents.
//! - House 5 got 60 presents.
//! - House 6 got 120 presents.
//! - House 7 got 80 presents.
//! - House 8 got 150 presents.
//! - House 9 got 130 presents.

//! The first house gets 10 presents: it is visited only by Elf 1, which delivers 1 * 10 = 10 presents. The fourth house gets 70 presents, because it is visited by Elves 1, 2, and 4, for a total of 10 + 20 + 40 = 70 presents.

//! What is the lowest house number of the house to get at least as many presents as the number in your puzzle input?

use aoc::PuzzleInput;

fn main() {
    let p = PuzzleInput::new("../data/20.txt");
    let n: u32 = p.string().parse().expect("Invalid u32");

    let part_1 = part_1(&n);
    println!("{part_1}");

    let part_2 = part_2(&n);
    println!("{part_2}");

}

fn part_1(n: &u32) -> u32 {
    let mut houses: Vec<usize> = vec![0; *n as usize /10 + 1];
    for elf in 1..houses.len() {
        let mut house = elf;
        while house < houses.len() {
            houses[house] += elf * 10;
            house += elf;
        }
    }
    for house in 0..houses.len() {
        if houses[house] >= *n as usize {
            return house as u32;
        }

    }
    panic!("No solution found");
}

fn part_2(n: &u32) -> u32 {
    let mut houses: Vec<usize> = vec![0; *n as usize / 11 + 1];
    for elf in 1..houses.len() {
        let mut house = elf;
        let mut count = 0;
        while house < houses.len() && count < 50 {
            houses[house] += elf * 11;
            house += elf;
            count += 1;
        }
    }
    for house in 0..houses.len() {
        if houses[house] >= *n as usize {
            return house as u32;
        }
    }
    panic!("No solution found")
}
