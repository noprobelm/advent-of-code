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
    let bag: HashMap<&str, u32> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    for line in lines {
        let split: Vec<&str> = line.split(": ").collect();

        let game_id = split[0]
            .chars()
            .filter(|c| c.is_digit(10))
            .map(|c| c.to_digit(10).expect("Invalid digit"))
            .fold(0, |acc, x| acc * 10 + x);

        let game: Vec<&str> = split[1]
            .split(" ")
            .map(|s| s.trim_matches(|c| c == ',' || c == ';'))
            .collect();

        if !game
            .iter()
            .skip(1)
            .step_by(2)
            .zip(game.iter().step_by(2))
            .any(|(c, n)| &n.parse::<u32>().expect("Invalid num") > bag.get(c).unwrap())
        {
            answer += game_id
        }
    }

    answer
}

fn part_2(lines: &Vec<&str>) -> u32 {
    let mut answer = 0;
    let bag: HashMap<&str, u32> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    for line in lines {
        let mut count: HashMap<&str, Vec<u32>> =
            HashMap::from([("red", vec![]), ("green", vec![]), ("blue", vec![])]);

        let split: Vec<&str> = line.split(": ").collect();
        let game_id = split[0]
            .chars()
            .filter(|c| c.is_digit(10))
            .map(|c| c.to_digit(10).expect("Invalid digit"))
            .fold(0, |acc, x| acc * 10 + x);

        let game: Vec<&str> = split[1]
            .split(" ")
            .map(|s| s.trim_matches(|c| c == ',' || c == ';'))
            .collect();

        for (color, n) in game.iter().skip(1).step_by(2).zip(game.iter().step_by(2)) {
            count
                .entry(color)
                .and_modify(|v| v.push(n.parse::<u32>().expect("Invalid num")));
        }

        answer += count
            .values()
            .fold(1, |acc, v| v.iter().max().unwrap() * acc);
    }
    answer
}
