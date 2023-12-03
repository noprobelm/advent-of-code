const BLACKLISTED: [u8; 3] = [105, 111, 108];

fn main() {
    let input = "vzbxkghb";
    let part_1 = part_1(input);
    println!("Part 1: {part_1}");

    let part_2 = part_2(&part_1);
    println!("Part 2: {part_2}");
}

fn part_1(password: &str) -> String {
    let password = password.to_string();
    let mut new = increment(&password);
    while valid_password(&new) == false {
        new = increment(&new);
    }
    new
}

fn part_2(password: &str) -> String {
    let password = password.to_string();
    let mut new = increment(&password);
    while valid_password(&new) == false {
        new = increment(&new);
    }
    new
}

fn increment(s: &String) -> String {
    let mut v: Vec<u8> = Vec::new();
    let bytes = s.as_bytes();
    for (i, b) in bytes.iter().enumerate().rev() {
        if *b == 122 {
            v.push(97);
        } else {
            v.push(*b + 1);
            for k in bytes[0..i].iter().rev() {
                v.push(*k);
            }
            break;
        }
    }
    v.iter()
        .map(|x| char::from_u32(*x as u32).unwrap())
        .rev()
        .collect::<String>()
}

fn valid_password(p: &String) -> bool {
    let bytes = p.as_bytes().to_vec();
    let mut has_three_consecutive = false;
    for (i, b) in bytes.iter().take(bytes.len() - 2).enumerate() {
        let trio: Vec<u8> = vec![*b, bytes[i + 1], bytes[i + 2]];

        if consecutive(&trio) == true {
            has_three_consecutive = true;
            break;
        }
    }
    has_three_consecutive == true
        && allowed_characters(&bytes) == true
        && has_unique_pairs(&bytes) == true
}

fn consecutive(bytes: &Vec<u8>) -> bool {
    for (i, b) in bytes.iter().enumerate().skip(1).rev() {
        if b - 1 != bytes[i - 1] {
            return false;
        }
    }
    true
}

fn allowed_characters(bytes: &Vec<u8>) -> bool {
    !bytes.iter().any(|&b| BLACKLISTED.contains(&b))
}

fn has_unique_pairs(bytes: &Vec<u8>) -> bool {
    let mut pairs = bytes
        .iter()
        .zip(bytes.iter().skip(1))
        .filter(|(x, y)| x == y)
        .collect::<Vec<(&u8, &u8)>>();

    pairs.sort();
    pairs.dedup();

    pairs.len() > 1
}
