use aoc::PuzzleInput;
use std::collections::HashMap;

fn main() {
    let p = PuzzleInput::new("../data/5.txt");

    let (map, page_updates) = parse_puzzle_input(p);

    let part_1 = part_1(&map, &page_updates);
    println!("Part 1: {part_1}");

    let part_2 = part_2(&map, &page_updates);
    println!("Part 2: {part_2}");
}

fn part_1(map: &HashMap<u32, Vec<u32>>, page_updates: &Vec<Vec<u32>>) -> u32 {
    page_updates.iter().fold(0, |acc, page| {
        if ordered(map, page).is_ok() {
            return acc + page.get(page.len() / 2).unwrap();
        }
        acc
    })
}

fn part_2(map: &HashMap<u32, Vec<u32>>, page_updates: &Vec<Vec<u32>>) -> u32 {
    page_updates.clone().iter_mut().fold(0, |acc, page| {
        let mut start_index = 0;
        let mut unsorted: bool = false;
        while let Err(failed_index) = ordered(&map, &page[start_index..]) {
            unsorted = true;
            let actual = start_index + failed_index;
            page.swap(actual, actual - 1);
            if start_index > 1 {
                start_index = actual - 2;
            }
        }
        if unsorted {
            return acc + page.get(page.len() / 2).unwrap();
        }
        acc
    })
}

fn ordered(map: &HashMap<u32, Vec<u32>>, page: &[u32]) -> Result<(), usize> {
    let mut explored: Vec<u32> = Vec::with_capacity(page.len());
    for (i, page) in page.iter().enumerate() {
        if let Some(ordered_before) = map.get(page) {
            if ordered_before.iter().any(|n| explored.contains(n)) {
                return Err(i);
            }
        }
        explored.push(*page);
    }
    Ok(())
}

fn parse_puzzle_input(puzzle_input: PuzzleInput) -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
    let mut map: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut page_updates: Vec<Vec<u32>> = Vec::new();

    let lines = puzzle_input.lines();
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

    (map, page_updates)
}
