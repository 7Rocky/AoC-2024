use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let (mut a_values, mut b_values): (Vec<i32>, Vec<i32>) = input
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let a: i32 = parts.next().unwrap().parse().expect("Invalid integer");
            let b: i32 = parts.next().unwrap().parse().expect("Invalid integer");
            (a, b)
        })
        .unzip();

    a_values.sort();
    b_values.sort();

    let mut s: i32 = a_values
        .iter()
        .zip(&b_values)
        .map(|(a, b)| (a - b).abs())
        .sum();

    println!("Total distance between lists (1): {s}");

    let mut b_map = HashMap::new();

    for b in b_values.iter() {
        if !b_map.contains_key(b) {
            b_map.insert(b, b_values.iter().filter(|&c| c == b).count() as i32);
        }
    }

    s = a_values
        .iter()
        .map(|a| a * b_map.get(a).unwrap_or(&0))
        .sum();

    println!("Similarity score (2): {s}");
}
