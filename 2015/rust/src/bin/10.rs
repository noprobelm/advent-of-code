fn main() {
    let input: u32 = 1321131112;
    let part_1 = part_1(&input);
    println!("Part 1: {part_1}");
    let part_2 = part_2(&input);
    println!("Part 2: {part_2}");
}

fn look_and_say(s: &String) -> String {
    let mut v: Vec<String> = Vec::new();
    let mut last = s.chars().nth(0).unwrap();
    let mut segment = String::new();
    for c in s.chars() {
        if c == last {
            segment.push(c);
        } else {
            v.push(segment.clone());
            last = c;
            segment = String::from(c);
        }
    }
    v.push(segment.clone());
    v.iter()
        .map(|n| vec![n.len().to_string(), n.chars().nth(0).unwrap().to_string()].join(""))
        .collect::<String>()
}
fn part_1(n: &u32) -> usize {
    let mut new: String = n.to_string();
    for i in 0..40 {
        new = look_and_say(&new);
    }
    new.len()
}

fn part_2(n: &u32) -> usize {
    let mut new: String = n.to_string();
    for i in 0..50 {
        new = look_and_say(&new);
    }
    new.len()
}
