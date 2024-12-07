use aoc::PuzzleInput;
use std::collections::HashMap;

fn main() {
    let p = PuzzleInput::new("../data/5.txt");
    let lines = p.lines();

    let part_1 = part_1(&lines);
    println!("Part 1: {part_1}");

    // let part_2 = part_2(&lines);
    // println!("Part 2: {part_2}");
}

fn part_1(lines: &Vec<&str>) -> u32 {
    let mut answer = 0;
    let mut map: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut page_updates: Vec<Vec<u32>> = Vec::new();

    let (first_section, second_section) =
        lines.split_at(lines.iter().position(|line| *line == "").unwrap());

    first_section.iter().for_each(|page_numbers_str| {
        let page_numbers: Vec<u32> = page_numbers_str
            .split("|")
            .map(|s| {
                s.parse::<u32>()
                    .expect("Found invalid u32 while parsing rules.")
            })
            .collect();

        map.entry(page_numbers[0])
            .and_modify(|v| v.push(page_numbers[1]))
            .or_insert(vec![page_numbers[1]]);
    });

    second_section.iter().skip(1).for_each(|updates_str| {
        let updates: Vec<u32> = updates_str
            .split(",")
            .map(|s| {
                s.parse::<u32>()
                    .expect("Found invalid u32 while parsing page updates.")
            })
            .collect();
        page_updates.push(updates)
    });

    page_updates.iter().for_each(|update| {
        let mut explored: Vec<u32> = Vec::with_capacity(update.len());
        let mut ordered: bool = true;
        update.iter().for_each(|page| {
            if let Some(ordered_before) = map.get(page) {
                if ordered_before.iter().any(|n| explored.contains(n)) {
                    ordered = false;
                }
            }

            explored.push(*page);
        });
        if ordered {
            answer += update.get(update.len() / 2).unwrap();
        }
    });

    answer
}

fn part_2(lines: &Vec<&str>) -> u32 {
    0
}
