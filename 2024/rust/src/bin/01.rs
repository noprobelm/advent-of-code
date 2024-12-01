use aoc::PuzzleInput;

fn main() {
    let p = PuzzleInput::new("../data/1.txt");
    let lines = p.lines();

    let part_1 = part_1(&lines);
    println!("Part 1: {part_1}");
}

fn part_1(lines: &Vec<&str>) -> u32 {
    let mut distance: i32 = 0;
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for line in lines {
        line.split("   ")
            .map(|n| n.parse::<i32>().expect("Not a valid u32!"))
            .collect::<Vec<i32>>()
            .iter()
            .enumerate()
            .for_each(|(i, n)| {
                if i % 2 == 0 {
                    right.push(*n)
                } else {
                    left.push(*n)
                }
            });
    }

    left.sort();
    right.sort();
    let zipped: Vec<(&i32, &i32)> = left.iter().zip(right.iter()).collect();
    zipped.iter().for_each(|(left_n, right_n)| {
        distance += (**left_n - **right_n).abs();
    });

    distance.try_into().unwrap()
}

