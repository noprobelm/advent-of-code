use aoc::PuzzleInput;
use std::collections::HashSet;
use std::hash::Hash;

fn main() {
    let p = PuzzleInput::new("../data/9.txt");
    let part_1 = part_1(&p);
    println!("Part 1: {}", part_1);
    let part_2 = part_2(&p);
    println!("Part 2: {}", part_2);
}

fn part_1(puzzle_input: &PuzzleInput) -> usize {
    let disk_map = puzzle_input.string();
    let mut translated = vec![];
    disk_map.chars().enumerate().step_by(2).for_each(|(i, c)| {
        let val = i / 2;
        let occupied_space = c.to_digit(10).unwrap();
        if i == disk_map.len() - 1 {
            (0..occupied_space).for_each(|_| {
                translated.push(val.to_string().chars().next().unwrap());
            });
        } else {
            let free_space = disk_map.chars().nth(i + 1).unwrap().to_digit(10).unwrap();
            (0..occupied_space).for_each(|_| {
                translated.push(val.to_string().chars().next().unwrap());
            });
            (0..free_space).for_each(|_| {
                translated.push('.');
            });
        }
    });
    for (i, c) in translated.clone().iter().enumerate().rev() {
        if *c != '.' {
            if let Some(match_idx) = translated.iter().position(|v| *v == '.') {
                translated.swap(match_idx, i);
            }
        }
        if !translated[0..i].iter().any(|v| *v == '.') {
            break;
        }
    }
    translated
        .iter()
        .filter(|&&c| c != '.')
        .collect::<Vec<&char>>()
        .iter()
        .enumerate()
        .fold(0, |acc, (i, x)| {
            acc + (x.to_digit(10).unwrap() as usize * i)
        })
}

fn part_2(puzzle_input: &PuzzleInput) -> usize {
    0
}
