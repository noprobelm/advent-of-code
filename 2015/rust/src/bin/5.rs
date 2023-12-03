use aoc::PuzzleInput;
use std::collections::HashMap;

fn main() {
    let p = PuzzleInput::new("../data/5.txt");
    let lines = p.lines();

    let part_1 = part_1(&lines);
    println!("Part 1: {part_1}");

    let part_2 = part_2(&lines);
    println!("Part 2: {part_2}");
}

/// Solves part 1 of the puzzle
/// A string must meet the following criteria to be counted as 'nice':
/// - It contains at least three vowels (aeiou only), like aei, xazegov, or aeiouaeiouaeiou.
/// - It contains at least one letter that appears twice in a row, like xx, abcdde (dd), or aabbccdd (aa, bb, cc, or dd).
/// - It does not contain the strings ab, cd, pq, or xy, even if they are part of one of the other requirements.

fn part_1(lines: &Vec<&str>) -> u32 {
    let mut nice: u32 = 0;
    for line in lines {
        let num_vowels = count_vowels(&line);
        let num_consecutive = num_consecutive_letters(&line);
        let blacklisted = contains_blacklisted(&line);
        if num_vowels >= 3 && num_consecutive > 0 && blacklisted == false {
            nice += 1;
        }
    }
    nice
}

/// Solves part 2 of the puzzle
/// A string must meet the following criteria to be counted as 'nice':
/// - It contains a pair of any two letters that appears at least twice in the string without overlapping, like xyxy (xy) or aabcdefgaa (aa), but not like aaa (aa, but it overlaps).
/// - It contains at least one letter which repeats with exactly one letter between them, like xyx, abcdefeghi (efe), or even aaa.

fn part_2(lines: &Vec<&str>) -> u32 {
    let mut nice: u32 = 0;
    for line in lines {
        let repeating_no_overlap = has_repeating_no_overlap(*line);
        let has_consecutive_divided = has_consecutive_with_divider(*line);

        if repeating_no_overlap == true && has_consecutive_divided == true {
            nice += 1;
        }
    }
    nice
}

fn count_vowels(s: &str) -> usize {
    let is_vowel = |c: char| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u');
    s.chars().filter(|&c| is_vowel(c)).count()
}

fn num_consecutive_letters(s: &str) -> usize {
    s.chars()
        .zip(s.chars().skip(1))
        .filter(|(x, y)| x == y)
        .count()
}

fn contains_blacklisted(s: &str) -> bool {
    let blacklisted = vec!["ab", "cd", "pq", "xy"];
    blacklisted
        .iter()
        .any(|&blacklisted| s.contains(blacklisted))
}

fn has_consecutive_with_divider(s: &str) -> bool {
    let bytes = s.as_bytes();
    for (i, _b) in bytes[..bytes.len() - 2].iter().enumerate() {
        if bytes[i] == bytes[i + 2] {
            return true;
        }
    }
    false
}

fn has_repeating_no_overlap(s: &str) -> bool {
    let mut matched: HashMap<(u8, u8), u8> = HashMap::new();
    let bytes = s.as_bytes();
    for (i, _b) in bytes[0..bytes.len() - 1].iter().enumerate() {
        let (left, right) = (bytes[i], bytes[i + 1]);
        if (left == right) && (i + 2 < bytes.len() && bytes[i + 2] == right) {
            continue;
        } else {
            matched
                .entry((left, right))
                .and_modify(|x| *x += 1)
                .or_insert(1);
        }

        if matched.values().any(|&x| x > 1) {
            return true;
        }
    }
    false
}
