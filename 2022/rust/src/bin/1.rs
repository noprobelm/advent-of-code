use aoc::PuzzleInput;

struct Rucksack {
    contents: Vec<u32>,
}

impl Rucksack {
    fn combined_calories(&self) -> u32 {
        let mut combined: u32 = 0;
        for i in &self.contents {
            combined += i;
        }
        combined
    }
}

fn main() {
    let p = PuzzleInput::new("../data/1.txt");
    let lines = p.lines();
    let rucksacks: Vec<Rucksack> = rucksacks_from_lines(lines);

    let mut combined: Vec<u32> = Vec::new();
    for rucksack in rucksacks {
        combined.push(rucksack.combined_calories())
    }

    let largest = find_largest(&combined);
    println!("Part 1: {largest}");

    let three_largest_combined = combine_three_largest(&combined);
    println!("Part 2: {three_largest_combined}");
}

fn rucksacks_from_lines(lines: Vec<&str>) -> Vec<Rucksack> {
    let mut rucksacks: Vec<Rucksack> = Vec::new();
    let mut contents: Vec<u32> = Vec::new();
    for i in lines {
        match i.parse() {
            Ok(num) => contents.push(num),
            Err(_) => {
                rucksacks.push(Rucksack {
                    contents: contents.clone(),
                });
                contents.clear();
            }
        }
    }
    rucksacks.push(Rucksack {
        contents: contents.clone(),
    });

    rucksacks
}

fn find_largest(calories: &Vec<u32>) -> u32 {
    *calories.iter().max().unwrap()
}

fn combine_three_largest(calories: &Vec<u32>) -> u32 {
    let mut sorted = calories.to_vec();
    sorted.sort_by(|a, b| b.cmp(a));
    sorted[..3].iter().sum()
}
