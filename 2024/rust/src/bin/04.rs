use aoc::PuzzleInput;

const PATTERN: &str = "XMAS";

fn main() {
    let p = PuzzleInput::new("../data/4.txt");
    let lines = p.lines();

    let part_1 = part_1(lines);
    println!("Part 1: {part_1}");

    // let part_2 = part_2(s);
    // println!("Part 2: {part_2}");
}

fn part_1(lines: Vec<&str>) -> u32 {
    let mut answer = 0;
    lines.iter().enumerate().for_each(|(row_pos, line)| {
        line.chars().enumerate().for_each(|(column_pos, c)| {
            if c == 'X' {
                for [x_vec, y_vec] in [
                    [0i32, 1i32],
                    [1i32, 1i32],
                    [1i32, 0i32],
                    [1i32, -1i32],
                    [0i32, -1i32],
                    [-1i32, -1i32],
                    [-1i32, 0i32],
                    [-1i32, 1i32],
                ] {
                    let mut matched = String::from(c);
                    for i in 1..=3 {
                        let x = (column_pos as i32 + i * x_vec as i32) as usize;
                        let y = (row_pos as i32 + i * y_vec as i32) as usize;
                        if let Some(row) = lines.get(y as usize) {
                            if let Some(char) = row.chars().nth(x as usize) {
                                if [x_vec, y_vec] == [1i32, -1i32] {}
                                matched.push(char);
                                if !PATTERN.contains(matched.as_str()) {
                                    break;
                                }
                            }
                        }
                    }
                    if PATTERN == matched.as_str() {
                        answer += 1;
                    }
                    matched.clear();
                }
            }
        });
    });
    answer
}

fn part_2(puzzle_input_str: &str) -> u32 {
    let mut answer = 0;
    answer
}
