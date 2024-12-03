use aoc::PuzzleInput;

fn main() {
    let p = PuzzleInput::new("../data/2.txt");
    let lines = p.lines();

    let part_1 = part_1(&lines);
    println!("Part 1: {part_1}");

    let part_2 = part_2(&lines);
    println!("Part 2: {part_2}");
}

fn within_range(n: &i32, n1: &i32) -> bool {
    let range = (n - n1).abs();
    range >= 1 && range <= 3
}

fn is_safe(row: Vec<i32>) -> (bool, Option<usize>) {
    let increasing: bool = row.get(0) < row.get(1);
    let (safe, i) = (0..row.len() - 1).fold((true, Some(0)), |safe, i| {
        let n = row.get(i).unwrap();
        let next = row.get(i + 1).unwrap();
        if n < next && !increasing || !within_range(n, next) {
            return (false, Some(i));
        } else if n > next && increasing || !within_range(n, next) {
            return (false, Some(i));
        }
        safe
    });

    if safe {
        return (safe, None);
    } else {
        return (safe, i);
    }
}

fn part_1(lines: &Vec<&str>) -> u32 {
    let mut answer = 0;

    lines.iter().for_each(|line| {
        let row: Vec<i32> = line
            .split(" ")
            .map(|n| n.parse::<i32>().expect("Not a valid i32!"))
            .collect();

        let (safe, _) = is_safe(row);
        if safe {
            answer += 1;
        }
    });
    answer
}

fn part_2(lines: &Vec<&str>) -> u32 {
    let mut answer = 0;

    lines.iter().for_each(|line| {
        let mut num_removed = 0;
        let mut safe = false;
        let mut to_remove: Option<usize> = None;

        let row: Vec<i32> = line
            .split(" ")
            .map(|n| n.parse::<i32>().expect("Not a valid i32!"))
            .collect();

        while !safe && num_removed < 2 {
            let mut row_to_test = row.clone();
            if let Some(to_remove) = to_remove {
                if num_removed == 0 {
                    row_to_test.remove(to_remove);
                } else if num_removed == 1 {
                    row_to_test.remove(to_remove + 1);
                }
                row_to_test.remove(to_remove);
            }
            (safe, to_remove) = is_safe(row_to_test.clone());
            if to_remove.is_some() {
                println!("{:?}", to_remove);
                num_removed += 1;
            }
        }

        if safe {
            answer += 1;
        }
    });
    answer
}
