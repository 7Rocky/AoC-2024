use std::fs;

use std::collections::HashSet;

fn next_nodes(start: (usize, usize), nodes: &Vec<Vec<u32>>, target: u32) -> Vec<(usize, usize)> {
    let mut next_nodes = Vec::new();

    if start.1 < nodes.len() - 1 && nodes[start.1 + 1][start.0] == target {
        next_nodes.push((start.0, start.1 + 1));
    }

    if start.1 > 0 && nodes[start.1 - 1][start.0] == target {
        next_nodes.push((start.0, start.1 - 1));
    }

    if start.0 < nodes[0].len() - 1 && nodes[start.1][start.0 + 1] == target {
        next_nodes.push((start.0 + 1, start.1));
    }

    if start.0 > 0 && nodes[start.1][start.0 - 1] == target {
        next_nodes.push((start.0 - 1, start.1));
    }

    next_nodes
}

fn dfs1(
    start: (usize, usize),
    nodes: &Vec<Vec<u32>>,
    score: &mut i32,
    visited: &mut HashSet<(usize, usize)>,
) {
    if visited.contains(&(start.0, start.1)) {
        return;
    } else {
        visited.insert((start.0, start.1));
    }

    if nodes[start.1][start.0] == 9 {
        *score += 1;
        return;
    }

    next_nodes(start, nodes, nodes[start.1][start.0] + 1)
        .iter()
        .for_each(|&n| dfs1(n, nodes, score, visited));
}

fn dfs2(start: (usize, usize), nodes: &Vec<Vec<u32>>, score: &mut i32) {
    if nodes[start.1][start.0] == 9 {
        *score += 1;
        return;
    }

    next_nodes(start, nodes, nodes[start.1][start.0] + 1)
        .iter()
        .for_each(|&n| dfs2(n, nodes, score));
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let map: Vec<Vec<u32>> = input
        .trim()
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut score = 0;
    let initial_nodes: HashSet<_> = map
        .iter()
        .enumerate()
        .flat_map(|(j, r)| {
            r.iter()
                .enumerate()
                .filter_map(move |(i, &n)| if n == 0 { Some((i, j)) } else { None })
        })
        .collect();

    initial_nodes
        .iter()
        .for_each(|&i| dfs1(i, &map, &mut score, &mut HashSet::<(usize, usize)>::new()));

    println!("Scores (1): {score}");

    score = 0;
    initial_nodes
        .iter()
        .for_each(|&i| dfs2(i, &map, &mut score));

    println!("Scores (2): {score}");
}
