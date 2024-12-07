use aoc::PuzzleInput;
use std::collections::HashSet;

const ALL_VECTORS: [[i32; 2]; 8] = [
    [0i32, 1i32],
    [1i32, 1i32],
    [1i32, 0i32],
    [1i32, -1i32],
    [0i32, -1i32],
    [-1i32, -1i32],
    [-1i32, 0i32],
    [-1i32, 1i32],
];

const DIAGONAL_VECTORS: [[i32; 2]; 4] =
    [[1i32, 1i32], [1i32, -1i32], [-1i32, -1i32], [-1i32, 1i32]];

fn main() {
    let p = PuzzleInput::new("../data/4.txt");
    let lines = p.lines();

    let part_1 = part_1(&lines);
    println!("Part 1: {part_1}");

    let part_2 = part_2(&lines);
    println!("Part 2: {part_2}");
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct URect {
    upper_left: (usize, usize),
    lower_right: (usize, usize),
}

impl URect {
    fn from_corners(first: (usize, usize), second: (usize, usize)) -> Self {
        let upper_left = (first.0.min(second.0), first.1.min(second.1));
        let lower_right = (first.0.max(second.0), first.1.max(second.1));

        if upper_left == lower_right {
            panic!("'X' and 'Y' must be different for each corner to build a valid URect");
        }

        URect {
            upper_left,
            lower_right,
        }
    }
}

fn contains_pattern(
    pattern: &str,
    matrix: &Vec<&str>,
    starting_position: (usize, usize),
    search_vector: (i32, i32),
) -> bool {
    let mut matched = String::from(
        matrix
            .get(starting_position.1)
            .unwrap()
            .chars()
            .nth(starting_position.0)
            .unwrap(),
    );
    for i in 1..pattern.len() as i32 {
        let x = (starting_position.0 as i32 + i * search_vector.0) as usize;
        let y = (starting_position.1 as i32 + i * search_vector.1) as usize;
        if let Some(row) = matrix.get(y as usize) {
            if let Some(char) = row.chars().nth(x as usize) {
                matched.push(char);
                if !pattern.contains(matched.as_str()) {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
    pattern == matched.as_str()
}

fn part_1(lines: &Vec<&str>) -> u32 {
    let mut answer = 0;
    lines.iter().enumerate().for_each(|(row_pos, line)| {
        line.chars().enumerate().for_each(|(column_pos, c)| {
            if c == 'X' {
                for [x_vec, y_vec] in ALL_VECTORS {
                    if contains_pattern("XMAS", &lines, (column_pos, row_pos), (x_vec, y_vec)) {
                        answer += 1;
                    }
                }
            }
        });
    });
    answer
}

fn part_2(lines: &Vec<&str>) -> u32 {
    let mut crosses_region: HashSet<URect> = HashSet::new();
    lines.iter().enumerate().for_each(|(row_pos, line)| {
        line.chars().enumerate().for_each(|(column_pos, c)| {
            if c == 'M' {
                for [x_vec, y_vec] in DIAGONAL_VECTORS {
                    if contains_pattern("MAS", &lines, (column_pos, row_pos), (x_vec, y_vec)) {
                        let (opposite_x, opposite_y) =
                            (column_pos as i32 + 2 * x_vec, row_pos as i32 + 2 * y_vec);
                        if opposite_x > 0
                            && contains_pattern(
                                "MAS",
                                &lines,
                                (opposite_x as usize, row_pos),
                                (-x_vec, y_vec),
                            )
                        {
                            crosses_region.insert(URect::from_corners(
                                (column_pos, row_pos),
                                (opposite_x as usize, opposite_y as usize),
                            ));
                        } else if opposite_y > 0
                            && contains_pattern(
                                "MAS",
                                &lines,
                                (column_pos, opposite_y as usize),
                                (x_vec, -y_vec),
                            )
                        {
                            crosses_region.insert(URect::from_corners(
                                (column_pos, row_pos),
                                (opposite_x as usize, opposite_y as usize),
                            ));
                        }
                    }
                }
            }
        });
    });
    crosses_region.len().try_into().unwrap()
}
