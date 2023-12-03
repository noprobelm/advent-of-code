//! # Advent of Code - 2015 Day 17: No Such Thing as Too Much
//!
//! ## Problem Description: Part 1
//! The elves bought too much eggnog again - 150 liters this time. To fit it all into your refrigerator, you'll need to
//! move it into smaller containers. You take an inventory of the capacities of the available containers.
//!
//! For example, suppose you have containers of size 20, 15, 10, 5, and 5 liters. If you need to store 25 liters, there
//! are four ways to do it:
//! - 15 and 10
//! - 20 and 5 (the first 5)
//! - 20 and 5 (the second 5)
//! - 15, 5, and 5
//!
//! Filling all containers entirely, how many different combinations of containers can exactly fit all 150 liters of
//! eggnog?
//!
//! ## Problem Description: Part 2
//! While playing with all the containers in the kitchen, another load of eggnog arrives! The shipping and receiving
//! department is requesting as many containers as you can spare.
//!
//! Find the minimum number of containers that can exactly fit all 150 liters of eggnog. How many different ways can you
//! fill that number of containers and still hold exactly 150 litres?
//!
//! In the example above, the minimum number of containers was two. There were three ways to use that many containers,
//! and so the answer there would be 3.

use aoc::PuzzleInput;

fn main() {
    let p = PuzzleInput::new("../data/17.txt");
    let lines: Vec<u32> = p
        .lines()
        .iter()
        .map(|s| s.parse::<u32>().expect("Invalid u32"))
        .collect();

    let part_1 = part_1(&lines);
    println!("Part 1: {part_1}");

    let part_2 = part_2(&lines);
    println!("Part 2: {part_2}")
}

/// Finds the number of subsets of numbers whose sum is equal to 150
fn part_1(nums: &Vec<u32>) -> u32 {
    let subsets = find_subset_sums(150, nums);
    subsets.len().try_into().unwrap()
}

/// Finds the minimum length subset of nums whose sum is equal to 150, then finds the number of possible subsets whose
/// length are equal to this number.
fn part_2(nums: &Vec<u32>) -> u32 {
    let subsets = find_subset_sums(150, nums);
    let sizes: Vec<usize> = subsets.iter().map(|v| v.len()).collect();
    sizes
        .iter()
        .filter(|v| v == &sizes.iter().min().unwrap())
        .count()
        .try_into()
        .unwrap()
}

/// Finds all subsets of numbers from a sequence of u32 whose sum are equal to 'sum'. This algorithm will recursively
/// find the subset of all nums whose sum are equal to the first u32 encountered that is equal to or less than the
/// sum itself, subtracted from the sum, until there are no more candidates (whew). Descendant subsets are iteratively
/// extended to the subset created by the calling function.
fn find_subset_sums(sum: u32, nums: &Vec<u32>) -> Vec<Vec<u32>> {
    let mut subsets: Vec<Vec<u32>> = Vec::new();
    if sum == 0 {
        subsets.push(Vec::new());
        return subsets;
    }

    for (i, &n) in nums.iter().enumerate() {
        let mut subset: Vec<u32> = Vec::new();
        if n <= sum {
            subset.push(n);
            let descendents = find_subset_sums(sum - n, &nums[i + 1..nums.len()].to_vec());
            for mut d in descendents {
                d.extend(subset.clone());
                subsets.push(d);
            }
        }
    }
    subsets
}
