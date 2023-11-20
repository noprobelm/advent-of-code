//! # Advent of Code - 2015 Day 2: I Was Told There Would Be No Math
//!
//! ## Problem Description: Part 1
//! The elves are running low on wrapping paper, and so they need to submit an order for more. They
//! have a list of the dimensions (length l, width w, and height h) of each present, and only want
//! to order exactly as much as they need.
//!
//! Fortunately, every present is a box (a perfect right rectangular prism), which makes calculating
//! the required wrapping paper for each gift a little easier: find the surface area of the box,
//! which is 2*l*w + 2*w*h + 2*h*l. The elves also need a little extra paper for each present: the
//! area of the smallest side.
//!
//! For example:
//!
//! - A present with dimensions `2x3x4` requires `2*6 + 2*12 + 2*8 = 52` square feet of wrapping paper plus `6` square feet of slack, for a total of `58` square feet.
//! - A present with dimensions `1x1x10` requires `2*1 + 2*10 + 2*10 = 42` square feet of wrapping paper plus `1` square foot of slack, for a total of `43` square feet.
//!
//! All numbers in the elves' list are in feet. How many total square feet of wrapping paper should they order?
//!
//! ## Problem Description: Part 2
//! The elves are also running low on ribbon. Ribbon is all the same width, so they only have to
//! worry about the length they need to order, which they would again like to be exact.
//!
//! The ribbon required to wrap a present is the shortest distance around its sides, or the smallest
//! perimeter of any one face. Each present also requires a bow made out of ribbon as well; the feet
//! of ribbon required for the perfect bow is equal to the cubic feet of volume of the present.
//! Don't ask how they tie the bow, though; they'll never tell.
//!
//! For example:
//!
//! - A present with dimensions `2x3x4` requires `2+2+3+3 = 10` feet of ribbon to wrap the present plus `2*3*4 = 24` feet of ribbon for the bow, for a total of `34` feet.
//! - A present with dimensions `1x1x10` requires `1+1+1+1 = 4` feet of ribbon to wrap the present plus `1*1*10 = 10` feet of ribbon for the bow, for a total of `14` feet.
//!
//! How many total feet of ribbon should they order?

use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

fn main() {
    let lines = lines_from_file("input.txt");

    let part_1 = part_1(&lines);
    println!("The sum of the surface area and the smallest side of all gifts from the puzzle input is: {part_1}");

    let part_2 = part_2(&lines);
    println!("The sum of the volume and the shortest perimeter length of all gifts from the puzzle input is: {part_2}");
}

/// Calculates the total amount of wrapping paper required as a function of the sum of the surface
/// area of all gifts and the smallest side of all gifts.
fn part_1(lines: &Vec<String>) -> u32 {
    let mut wrapping_paper: u32 = 0;
    for line in lines {
        let mut dimensions: Vec<u32> = line
            .split("x")
            .map(|d| d.parse::<u32>().expect("Not a valid u32!"))
            .collect();

        let xmas_gift = RectPrism::new(dimensions[0], dimensions[1], dimensions[2]);
        wrapping_paper += xmas_gift.smallest_side();
        wrapping_paper += xmas_gift.surface_area();
    }
    wrapping_paper
}

/// Calculates the total amount of ribbon length required as a function of the sum of the volume of
/// all gifts and the shortest perimeter of all gifts.
fn part_2(lines: &Vec<String>) -> u32 {
    let mut ribbon: u32 = 0;
    for line in lines {
        let mut dimensions: Vec<u32> = line
            .split("x")
            .map(|d| d.parse::<u32>().expect("Not a valid u32!"))
            .collect();

        let xmas_gift = RectPrism::new(dimensions[0], dimensions[1], dimensions[2]);
        ribbon += xmas_gift.volume();
        ribbon += xmas_gift.shortest_perimeter();
    }
    ribbon
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

/// Abstraction for a rectangular prism. `a`, `b`, and `c` are the dimensions. This struct relies
/// on `a`, `b`, and `c` being in ascending order. The `new` method will handle this automatically.
struct RectPrism {
    a: u32,
    b: u32,
    c: u32,
}

impl RectPrism {
    fn new(a: u32, b: u32, c: u32) -> Self {
        let mut dimensions = vec![a, b, c];
        dimensions.sort();
        Self { a: a, b: b, c: c }
    }
    fn surface_area(&self) -> u32 {
        2 * self.a * self.b + 2 * self.b * self.c + 2 * self.c * self.a
    }

    fn shortest_perimeter(&self) -> u32 {
        self.a * 2 + self.b * 2
    }

    fn volume(&self) -> u32 {
        self.a * self.b * self.c
    }

    fn smallest_side(&self) -> u32 {
        self.a * self.b
    }
}
