use md5;
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
