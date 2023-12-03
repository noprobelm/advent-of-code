use aoc::PuzzleInput;
use std::collections::HashSet;

fn main() {
    let p = PuzzleInput::new("../data/6.txt");
    let s = p.string();
    let part_1 = part_1(s);
    println!("Part 1: {part_1}");

    let part_2 = part_2(s);
    println!("Part 2: {part_2}");
}

fn part_1(s: &str) -> usize {
    for i in 0..s.len() {
        let end = if i + 4 < s.len() + 1 { i + 4 } else { break };
        if contains_duplicates(&s[i..end], 4) == false {
            return end;
        }
    }
    0
}

fn part_2(s: &str) -> usize {
    for i in 0..s.len() {
        let end = if i + 14 < s.len() + 1 {
            i + 14
        } else {
            break;
        };
        if contains_duplicates(&s[i..end], 14) == false {
            return end;
        }
    }
    0
}
fn contains_duplicates(v: &str, n: usize) -> bool {
    let set: HashSet<char> = HashSet::from_iter(v.chars());
    if set.len() == n {
        return false;
    }
    true
}
