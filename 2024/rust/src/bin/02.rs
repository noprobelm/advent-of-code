use aoc::PuzzleInput;

fn main() {
    let p = PuzzleInput::new("../data/2.txt");
    let lines = p.lines();

    let part_1 = part_1(&lines);
    println!("Part 1: {part_1}");

    let part_2 = part_2(&lines);
    println!("Part 2: {part_2}");
}

/// Checks if two numbers are within range of our provided rules.
fn within_range(n: &i32, n1: &i32) -> bool {
    let range = (n - n1).abs();
    range >= 1 && range <= 3
}

/// If a row fails the provided rules, return the violating index. Otherwise return None.
fn fails_by(row: Vec<i32>) -> Option<usize> {
    let increasing: bool = row.get(0) < row.get(1);
    let mut failed_by: Option<usize> = None;
    row.iter().enumerate().for_each(|(i, n)| {
        if i == row.len() - 1 {
            return;
        }
        let next = row.get(i + 1).unwrap();
        if n < next && !increasing || !within_range(n, next) {
            failed_by = Some(i);
        } else if n > next && increasing || !within_range(n, next) {
            failed_by = Some(i);
        }
    });

    return failed_by;
}

fn part_1(lines: &Vec<&str>) -> u32 {
    let mut answer = 0;

    lines.iter().for_each(|line| {
        let row: Vec<i32> = line
            .split(" ")
            .map(|n| n.parse::<i32>().expect("Not a valid i32!"))
            .collect();

        if fails_by(row).is_none() {
            answer += 1;
        }
    });
    answer
}

fn part_2(lines: &Vec<&str>) -> u32 {
    let mut answer = 0;

    lines.iter().for_each(|line| {
        let row: Vec<i32> = line
            .split(" ")
            .map(|n| n.parse::<i32>().expect("Not a valid i32!"))
            .collect();

        let mut failed_by = fails_by(row.clone());
        if failed_by.is_some() {
            for i in 0..row.len() {
                let mut row_to_test = row.clone();
                row_to_test.remove(i);
                failed_by = fails_by(row_to_test);
                if failed_by.is_none() {
                    answer += 1;
                    break;
                }
            }
        } else {
            answer += 1;
            return;
        }
    });

    answer
}
