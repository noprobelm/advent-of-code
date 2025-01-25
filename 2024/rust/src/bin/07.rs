use aoc::PuzzleInput;

fn main() {
    let p = PuzzleInput::new("../data/6.txt");
    let input = vec![1, 2, 3];
    let perms = permutations(input);

    for perm in perms {
        println!("{:?}", perm);
    }
}

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

fn permutations<T: Clone>(input: Vec<T>) -> Vec<Vec<T>> {
    let mut result = Vec::new();
    let mut arr = input.clone();
    generate_permutations(&mut arr, 0, &mut result);
    result
}

enum Operation {
    Product,
    Sum,
}
