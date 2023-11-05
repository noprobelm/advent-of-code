use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

fn main() {
    let lines = lines_from_file("input.txt");

    let part_1 = part_1(&lines);
    println!("{part_1}");

    let part_2 = part_2(&lines);
    println!("{part_2}");
}

fn part_1(lines: &Vec<String>) -> u32 {
    let mut wrapping_paper: u32 = 0;
    for line in lines {
        let mut dimensions: Vec<u32> = line
            .split("x")
            .map(|d| d.parse::<u32>().expect("Not a valid u32!"))
            .collect();
        dimensions.sort();

        let xmas_gift = RectPrism {
            a: dimensions[0],
            b: dimensions[1],
            c: dimensions[2],
        };
        wrapping_paper += xmas_gift.smallest_side();
        wrapping_paper += xmas_gift.surface_area();
    }
    wrapping_paper
}

fn part_2(lines: &Vec<String>) -> u32 {
    let mut ribbon: u32 = 0;
    for line in lines {
        let mut dimensions: Vec<u32> = line
            .split("x")
            .map(|d| d.parse::<u32>().expect("Not a valid u32!"))
            .collect();
        dimensions.sort();
        let xmas_gift = RectPrism {
            a: dimensions[0],
            b: dimensions[1],
            c: dimensions[2],
        };
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

struct RectPrism {
    a: u32,
    b: u32,
    c: u32,
}

impl RectPrism {
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
