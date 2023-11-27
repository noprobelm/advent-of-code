//! # Advent of Code - 2015 Day 4: The Ideal Stocking Stuffer
//!
//! ## Problem Description: Part 1
//! Santa needs help mining some AdventCoins (very similar to bitcoins) to use as gifts for all the economically
//! forward-thinking little girls and boys.
//!
//! To do this, he needs to find MD5 hashes which, in hexadecimal, start with at least five zeroes. The input to the MD5
//! hash is some secret key (your puzzle input, given below) followed by a number in decimal. To mine AdventCoins, you
//! must find Santa the lowest positive number (no leading zeroes: 1, 2, 3, ...) that produces such a hash.
//!
//! For example:
//! - If your secret key is abcdef, the answer is 609043, because the MD5 hash of abcdef609043 starts with five zeroes (000001dbbfa...), and it is the lowest such number to do so.
//! - If your secret key is pqrstuv, the lowest number it combines with to make an MD5 hash starting with five zeroes is 1048970; that is, the MD5 hash of pqrstuv1048970 looks like 000006136ef....
//!
//! ## Problem Description: Part 2
//! Now find one that starts with six zeroes.use md5;

use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

fn main() {
    let secret = "ckczppom";
    part_1(secret);
    part_2(secret);
}

fn part_1(secret: &str) {
    let mut n: u32 = 1;
    //    while md5::compute(b"{secret}{n}")[0..5] !=
    let mut data = md5::compute(format!("{}{}", secret, n));
    let mut digest = format!("{:?}", data);
    while &digest[0..5] != "00000" {
        n += 1;
        data = md5::compute(format!("{}{}", secret, n));
        digest = format!("{:?}", data);
    }
    println!("The lowest number to produce an md5 hash with 5 preceding zeros when appended to secret '{secret}' is {n}. The full md5 for {secret} is '{:?}{n}'", digest);
}

fn part_2(secret: &str) {
    let mut n: u32 = 1;
    //    while md5::compute(b"{secret}{n}")[0..5] !=
    let mut data = md5::compute(format!("{}{}", secret, n));
    let mut digest = format!("{:?}", data);
    while &digest[0..6] != "000000" {
        n += 1;
        data = md5::compute(format!("{}{}", secret, n));
        digest = format!("{:?}", data);
    }
    println!("The lowest number to produce an md5 hash with 6 preceding zeros when appended to secret '{secret}' is {n}. The full md5 for {secret} is '{:?}{n}'", digest);
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
