use aoc::PuzzleInput;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn main() {
    let p = PuzzleInput::new("../data/9.txt");
    let lines = p.lines();
    let part_1 = part_1(&lines);
    println!("Part 1: {part_1}");

    let part_2 = part_2(&lines);
    println!("Part 2: {part_2}")
}

fn part_1(lines: &Vec<&str>) -> usize {
    // tsp == 'traveling salesman problem'
    let mut tsp_routes: Vec<usize> = Vec::new();
    let mut routes: HashMap<(&str, &str), usize> = HashMap::new();
    for line in lines {
        let split: Vec<&str> = line.split(" ").collect();
        let distance = split
            .last()
            .unwrap()
            .parse::<usize>()
            .expect("Attempted to parse to usize from incompatible type");
        routes.entry((split[0], split[2])).or_insert(distance);
        routes.entry((split[2], split[0])).or_insert(distance);
    }
    let locations: Vec<&str> = routes
        .keys()
        .map(|x| x.0)
        .collect::<HashSet<&str>>()
        .into_iter()
        .collect();
    let permutations = locations.clone().into_iter().permutations(locations.len());
    for p in permutations {
        let mut distance: usize = 0;
        for (i, f) in p.iter().enumerate().take(p.len() - 1) {
            let t = p[i + 1];
            distance += routes.get(&(f, t)).unwrap();
        }
        tsp_routes.push(distance);
    }

    *tsp_routes.iter().min().unwrap()
}

fn part_2(lines: &Vec<&str>) -> usize {
    // tsp == 'traveling salesman problem'
    let mut tsp_routes: Vec<usize> = Vec::new();
    let mut routes: HashMap<(&str, &str), usize> = HashMap::new();
    for line in lines {
        let split: Vec<&str> = line.split(" ").collect();
        let distance = split
            .last()
            .unwrap()
            .parse::<usize>()
            .expect("Attempted to parse to usize from incompatible type");
        routes.entry((split[0], split[2])).or_insert(distance);
        routes.entry((split[2], split[0])).or_insert(distance);
    }
    let locations: Vec<&str> = routes
        .keys()
        .map(|x| x.0)
        .collect::<HashSet<&str>>()
        .into_iter()
        .collect();
    let permutations = locations.clone().into_iter().permutations(locations.len());
    for p in permutations {
        let mut distance: usize = 0;
        for (i, f) in p.iter().enumerate().take(p.len() - 1) {
            let t = p[i + 1];
            distance += routes.get(&(f, t)).unwrap();
        }
        tsp_routes.push(distance);
    }

    *tsp_routes.iter().max().unwrap()
}
