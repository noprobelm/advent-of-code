use aoc::PuzzleInput;
use std::collections::HashSet;
use std::hash::Hash;

fn main() {
    let p = PuzzleInput::new("../data/7.txt");
    let part_1 = part_1(&p);
    println!("Part 1: {}", part_1);
    let part_2 = part_2(&p);
    println!("Part 2: {}", part_2);
}

fn part_1(puzzle_input: &PuzzleInput) -> u64 {
    let mut part_1_answer = 0;
    'outer: for line in puzzle_input.lines() {
        let mut operations: Vec<Operation> = Vec::new();
        let split: Vec<&str> = line.split(" ").collect();
        let solution = parse_solution(split[0]);
        let parts = parse_equation_parts(&split[1..]);
        for _ in 0..parts.len() - 1 {
            operations.push(Operation::Product);
        }
        let mut new_operations = operations.clone();
        for i in 0..=operations.len() {
            let perms = unique_permutations(new_operations.clone());
            for perm in perms {
                let answer =
                    (0..parts.len() - 1).fold(parts[0], |acc, k| perm[k].solve(acc, parts[k + 1]));
                if answer == solution {
                    part_1_answer += solution;
                    continue 'outer;
                }
            }
            if i < operations.len() {
                new_operations[i] = Operation::Sum;
            }
        }
    }
    part_1_answer
}

fn part_2(puzzle_input: &PuzzleInput) -> u64 {
    let mut part_2_answer = 0;
    'outer: for line in puzzle_input.lines() {
        println!("{:?}", line);
        let split: Vec<&str> = line.split(" ").collect();
        let solution = parse_solution(split[0]);
        let parts = parse_equation_parts(&split[1..]);
        for perm in permutations_of_3(parts.len() - 1) {
            let answer =
                (0..parts.len() - 1).fold(parts[0], |acc, k| perm[k].solve(acc, parts[k + 1]));
            if answer == solution {
                part_2_answer += solution;
                continue 'outer;
            }
        }
    }
    part_2_answer
}

fn parse_solution(s: &str) -> u64 {
    s[..&s.len() - 1]
        .parse::<u64>()
        .expect("Invalid u64 found while parsing solution.'")
}

fn parse_equation_parts(parts: &[&str]) -> Vec<u64> {
    parts
        .iter()
        .map(|n| {
            n.parse::<u64>()
                .expect("Invalid u64 found while parsing solution parts")
        })
        .collect()
}

#[allow(unused)]
fn generate_permutations<T: Clone>(arr: &mut Vec<T>, start: usize, result: &mut Vec<Vec<T>>) {
    if start == arr.len() - 1 {
        result.push(arr.clone());
        return;
    }

    for i in start..arr.len() {
        arr.swap(start, i);
        generate_permutations(arr, start + 1, result);
        arr.swap(start, i);
    }
}

#[allow(unused)]
fn permutations<T: Clone>(input: Vec<T>) -> Vec<Vec<T>> {
    let mut result = Vec::new();
    let mut arr = input.clone();
    generate_permutations(&mut arr, 0, &mut result);
    result
}

fn generate_unique_permutations<T: Clone + Hash + Eq>(
    arr: Vec<T>,
    start: usize,
    result: &mut Vec<Vec<T>>,
) {
    if start == arr.len() - 1 {
        result.push(arr.clone());
        return;
    }

    let set: HashSet<T> = arr.clone().into_iter().collect();
    for first_element in set {
        let mut remaining_elements = arr.clone();
        remaining_elements.remove(
            remaining_elements
                .iter()
                .position(|x| *x == first_element)
                .unwrap(),
        );
        for sub_permutation in unique_permutations(remaining_elements) {
            let mut sub_result: Vec<T> = Vec::new();
            sub_result.push(first_element.clone());
            sub_result.extend(sub_permutation);
            result.push(sub_result);
        }
    }
}

fn unique_permutations<T: Clone + Hash + Eq>(input: Vec<T>) -> Vec<Vec<T>> {
    let mut result = Vec::new();
    generate_unique_permutations(input, 0, &mut result);
    result
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
enum Operation {
    Product,
    Sum,
    Concatenate,
}

impl Operation {
    fn solve(&self, n1: u64, n2: u64) -> u64 {
        match self {
            Operation::Product => n1 * n2,
            Operation::Sum => n1 + n2,
            Operation::Concatenate => format!("{n1}{n2}")
                .parse::<u64>()
                .expect("Invalid u64 while performing number concatenation operation."),
        }
    }
}

fn permutations_of_3(n: usize) -> Vec<Vec<Operation>> {
    let mut result = Vec::new();
    let variables = vec![Operation::Product, Operation::Sum, Operation::Concatenate];
    generate_permutations2(&variables, n, &mut Vec::new(), &mut result);
    result
}

fn generate_permutations2(
    variables: &[Operation],
    n: usize,
    current: &mut Vec<Operation>,
    result: &mut Vec<Vec<Operation>>,
) {
    if current.len() == n {
        result.push(current.clone());
        return;
    }

    for &var in variables {
        current.push(var);
        generate_permutations2(variables, n, current, result);
        current.pop();
    }
}
