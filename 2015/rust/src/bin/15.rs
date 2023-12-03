//! # Advent of Code - 2015 Day 15: Science for Hungry People
//!
//! ## Problem Description: Part 1
//! Today, you set out on the task of perfecting your milk-dunking cookie recipe. All you have to do is find the right
//! balance of ingredients.
//!
//! Your recipe leaves room for exactly 100 teaspoons of ingredients. You make a list of the remaining ingredients you
//! could use to finish the recipe (your puzzle input) and their properties per teaspoon:
//!
//! - capacity (how well it helps the cookie absorb milk)
//! - durability (how well it keeps the cookie intact when full of milk)
//! - flavor (how tasty it makes the cookie)
//! - texture (how it improves the feel of the cookie)
//! - calories (how many calories it adds to the cookie)
//!
//! You can only measure ingredients in whole-teaspoon amounts accurately, and you have to be accurate so you can
//! reproduce your results in the future. The total score of a cookie can be found by adding up each of the
//! properties (negative totals become 0) and then multiplying together everything except calories.
//!
//! For instance, suppose you have these two ingredients:
//!
//! - Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
//! - Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
//!
//! Then, choosing to use 44 teaspoons of butterscotch and 56 teaspoons of cinnamon (because the amounts of each
//! ingredient must add up to 100) would result in a cookie with the following properties:
//!
//! - A capacity of 44*-1 + 56*2 = 68
//! - A durability of 44*-2 + 56*3 = 80
//! - A flavor of 44*6 + 56*-2 = 152
//! - A texture of 44*3 + 56*-1 = 76
//!
//! Multiplying these together (68 * 80 * 152 * 76, ignoring calories for now) results in a total score of 62842880,
//! which happens to be the best score possible given these ingredients. If any properties had produced a negative
//! total, it would have instead become zero, causing the whole score to multiply to zero.
//!
//! Given the ingredients in your kitchen and their properties, what is the total score of the highest-scoring cookie you can make?
//!
//! ## Problem Description: Part 2
//! Your cookie recipe becomes wildly popular! Someone asks if you can make another recipe that has exactly 500 calories
//! per cookie (so they can use it as a meal replacement). Keep the rest of your award-winning process the same (100
//! teaspoons, same ingredients, same scoring system).
//!
//! For example, given the ingredients above, if you had instead selected 40 teaspoons of butterscotch and 60 teaspoons
//! of cinnamon (which still adds to 100), the total calorie count would be 40*8 + 60*3 = 500. The total score would go
//! down, though: only 57600000, the best you can do in such trying circumstances.
//!
//! Given the ingredients in your kitchen and their properties, what is the total score of the highest-scoring cookie you
//! can make with a calorie total of 500?

use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::iter::IntoIterator;

fn main() {
    let ingredients: Vec<Ingredient> = vec![
        Ingredient::Sugar,
        Ingredient::Sprinkles,
        Ingredient::Candy,
        Ingredient::Chocolate,
    ];

    let combinations: Vec<Vec<Ingredient>> = combinations_no_repitition(ingredients, 100);
    let _part_1 = part_1(combinations.clone());
    println!("Part 1: {:?}", combinations.len());

    let part_2 = part_2(combinations.clone());
    println!("Part 2: {:?}", part_2)
}

// This algorithm takes any type with the trait IntoIterator, creates a set of its contents, and generates all possible
// combinations without repition. This is achieved using a HashMap<usize, Item>, where the usize key is used to keep
// track of what we've visited so far.
//
// While this function has the advantage of calculating the number of possible combinations of a set of 'n' elements,
// computational speed suffers due to our constant referencing of the HashMap keys. A solution to this could be
// through use of a crate like 'strum', which would allow us to iterate through enum variants.
//
// Adapting the function to accept any type meeting the generic parameters was a fun exercise. Might improve this
// later.
fn combinations_no_repitition<Item, Container>(items: Container, n: usize) -> Vec<Vec<Item>>
where
    Item: Hash + Debug + Copy,
    Container: IntoIterator<Item = Item>,
{
    let h: HashMap<usize, Item> = items.into_iter().enumerate().collect();
    let mut c: Vec<usize> = (0..n).map(|_| 0).collect();
    let mut combinations: Vec<Vec<Item>> =
        Vec::from([c.clone().iter().map(|k| *h.get(&k).unwrap()).collect()]);

    let num_unique = h.len();
    let mut current: usize;
    while c[n - 1] != num_unique - 1 {
        current = 0;
        while c[current] == num_unique - 1 {
            current += 1;
        }
        let reset_val = c[current] + 1;
        for i in 0..=current {
            c[i] = reset_val;
        }
        combinations.push(c.clone().iter().map(|k| *h.get(&k).unwrap()).collect());
    }
    combinations
}

// Calculates the total score for each combination of ingredients (calories exclusive)
fn part_1(combinations: Vec<Vec<Ingredient>>) -> i32 {
    let mut scores: Vec<i32> = Vec::new();
    for v in combinations {
        let (mut capacity, mut durability, mut flavor, mut texture) = (0, 0, 0, 0);
        for i in v {
            capacity += i.attributes().capacity;
            durability += i.attributes().durability;
            flavor += i.attributes().flavor;
            texture += i.attributes().texture;
        }
        if capacity < 0 {
            capacity = 0
        };
        if durability < 0 {
            durability = 0
        };
        if flavor < 0 {
            flavor = 0
        };
        if texture < 0 {
            texture = 0
        };
        scores.push(capacity * durability * flavor * texture);
    }

    *scores.iter().max().unwrap()
}

// Calculates the total score for each combination of ingredients (calories inclusive). Returns the highest scoring
// cookie whose calorie count is also exactly '500'
fn part_2(combinations: Vec<Vec<Ingredient>>) -> i32 {
    let mut scores: Vec<i32> = Vec::new();
    for v in combinations {
        let (mut capacity, mut durability, mut flavor, mut texture, mut calories) = (0, 0, 0, 0, 0);
        for i in v {
            capacity += i.attributes().capacity;
            durability += i.attributes().durability;
            flavor += i.attributes().flavor;
            texture += i.attributes().texture;
            calories += i.attributes().calories;
        }
        if capacity < 0 {
            capacity = 0
        };
        if durability < 0 {
            durability = 0
        };
        if flavor < 0 {
            flavor = 0
        };
        if texture < 0 {
            texture = 0
        };
        if calories < 0 {
            calories = 0
        }
        if calories == 500 {
            scores.push(capacity * durability * flavor * texture)
        };
    }

    *scores.iter().max().unwrap()
}

// Attributes struct for ingredients
#[derive(Copy, Clone, Debug, Hash)]
struct Attributes {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl Attributes {
    fn new(capacity: i32, durability: i32, flavor: i32, texture: i32, calories: i32) -> Self {
        Attributes {
            capacity,
            durability,
            flavor,
            texture,
            calories,
        }
    }
}

// All possible ingredient types
#[derive(Copy, Clone, Debug, Hash)]
enum Ingredient {
    Sugar,
    Sprinkles,
    Candy,
    Chocolate,
}

impl Ingredient {
    fn attributes(&self) -> Attributes {
        match *self {
            Ingredient::Sugar => Attributes::new(3, 0, 0, -3, 2),
            Ingredient::Sprinkles => Attributes::new(-3, 3, 0, 0, 9),
            Ingredient::Candy => Attributes::new(-1, 0, 4, 0, 1),
            Ingredient::Chocolate => Attributes::new(0, 0, -2, 2, 8),
        }
    }
}
