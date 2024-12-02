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
    lines.iter().for_each(|line| {
        let row: Vec<u32> = line
            .split(" ")
            .map(|n| n.parse::<u32>().expect("Not a valid u32!"))
            .collect();

        let mut safe: bool = true;
        let increasing: bool = row.get(0).unwrap() < row.get(1).unwrap();

        for i in 0..row.len() - 1 {
            let n = row.get(i).unwrap();
            let next = row.get(i + 1).unwrap();
            if n < next && increasing {
                let range = next - n;
                if range >= 1 && range <= 3 {
                    continue;
                } else {
                    safe = false;
                    break;
                }
            } else if n > next && !increasing {
                let range = n - next;
                if range >= 1 && range <= 3 {
                    continue;
                } else {
                    safe = false;
                    break;
                }
            } else {
                println!("{:?}", row);
                safe = false;
                break;
            }
        }

        if safe == true {
            // println!("{:?}", row);
            answer += 1;
        }
    });
    answer
}

fn part_2(lines: &Vec<&str>) -> u32 {
    let mut answer = 0;
    answer
}
