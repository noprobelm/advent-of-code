use aoc::PuzzleInput;
use std::collections::HashMap;

fn main() {
    let p = PuzzleInput::new("../data/2.txt");
    let lines = p.lines();

    let part_1 = part_1(&lines);
    println!("Part 1: {part_1}");

    let part_2 = part_2(&lines);
    println!("Part 2: {part_2}");
}

fn part_1(lines: &Vec<&str>) -> u32 {
    let mut answer = 0;
    let within_range = |n: &i32, n1: &i32| -> bool {
        let range = (n - n1).abs();
        if range >= 1 && range <= 3 {
            return true;
        } else {
            return false;
        }
    };

    lines.iter().for_each(|line| {
        let row: Vec<i32> = line
            .split(" ")
            .map(|n| n.parse::<i32>().expect("Not a valid i32!"))
            .collect();

        let increasing: bool = row.get(0) < row.get(1);
        let safe: bool = (0..row.len() - 1).fold(true, |safe, i| {
            let n = row.get(i).unwrap();
            let next = row.get(i + 1).unwrap();
            if n < next && increasing {
                if !within_range(n, next) {
                    return false;
                }
            } else if n > next && !increasing {
                if !within_range(n, next) {
                    return false;
                }
            } else {
                return false;
            }
            safe
        });

        if safe {
            answer += 1;
        }
    });
    answer
}

fn part_2(lines: &Vec<&str>) -> u32 {
    let mut answer = 0;
    answer
}
