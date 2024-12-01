use aoc::PuzzleInput;

fn main() {
    let p = PuzzleInput::new("../data/1.txt");
    let lines = p.lines();

    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();
    lines.iter().for_each(|line| {
        line.split("   ")
            .map(|n| n.parse::<u32>().expect("Not a valid u32!"))
            .collect::<Vec<u32>>()
            .iter()
            .enumerate()
            .for_each(|(i, n)| {
                if i % 2 == 0 {
                    right.push(*n)
                } else {
                    left.push(*n)
                }
            });
    });

    let part_1 = part_1(&left, &right);
    println!("Part 1: {part_1}");

    let part_2 = part_2(&left, &right);
    println!("Part 2: {part_2}");
}

fn part_1(left: &Vec<u32>, right: &Vec<u32>) -> u32 {
    let mut left = left.clone();
    let mut right = right.clone();

    left.sort();
    right.sort();

    left.iter()
        .zip(right.iter())
        .collect::<Vec<(&u32, &u32)>>()
        .iter()
        .fold(0, |acc, (left_n, right_n)| {
            if left_n > right_n {
                return acc + **left_n - **right_n;
            } else {
                return acc + **right_n - **left_n;
            }
        })
}

fn part_2(left: &Vec<u32>, right: &Vec<u32>) -> u32 {
    left.iter().fold(0, |acc, left_n| {
        acc + left_n
            * (right
                .iter()
                .filter(|right_n| *right_n == left_n)
                .collect::<Vec<&u32>>()
                .len() as u32)
    })
}
