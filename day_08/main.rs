use std::fs;

use std::collections::{HashMap, HashSet};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let lines: Vec<_> = input.trim().lines().collect();

    let mut all_antennas = HashSet::new();
    let mut antennas = HashMap::new();

    for j in 0..lines.len() {
        for i in 0..lines[j].len() {
            if lines[j].chars().nth(i).unwrap() != '.' {
                antennas
                    .entry(lines[j].chars().nth(i).unwrap())
                    .or_insert_with(Vec::new)
                    .push((i as i32, j as i32));
            }
        }
    }

    antennas.clone().into_iter().for_each(|(_, positions)| {
        (0..positions.len()).for_each(|i| {
            all_antennas.insert(positions[i]);
        });
    });

    let mut antinodes = HashSet::new();

    for (_, positions) in antennas.clone().into_iter() {
        for ii in 0..positions.len() {
            let pp = positions[ii];

            for i in 0..positions.len() {
                let p = positions[i];

                if p.0 == pp.0 && p.1 == pp.1 {
                    continue;
                }

                let a = (2 * pp.0 - p.0, 2 * pp.1 - p.1);
                let b = (2 * p.0 - pp.0, 2 * p.1 - pp.1);

                if 0 <= a.0 && a.0 < lines[0].len() as i32 && 0 <= a.1 && a.1 < lines.len() as i32 {
                    antinodes.insert(a);
                }

                if 0 <= b.0 && b.0 < lines[0].len() as i32 && 0 <= b.1 && b.1 < lines.len() as i32 {
                    antinodes.insert(b);
                }
            }
        }
    }

    println!("Unique locations (1): {}", antinodes.len());

    antinodes = HashSet::new();

    for (_, positions) in antennas.clone().into_iter() {
        for ii in 0..positions.len() {
            let pp = positions[ii];

            for i in 0..positions.len() {
                let p = positions[i];

                if p.0 == pp.0 && p.1 == pp.1 {
                    continue;
                }

                let diff = (p.0 - pp.0, p.1 - pp.1);

                let mut m = 1;
                let mut a = (pp.0 - m * diff.0, pp.1 - m * diff.1);

                while 0 <= a.0
                    && a.0 < lines[0].len() as i32
                    && 0 <= a.1
                    && a.1 < lines.len() as i32
                {
                    m += 1;
                    if !all_antennas.contains(&a) {
                        antinodes.insert(a);
                    }
                    a = (pp.0 - m * diff.0, pp.1 - m * diff.1);
                }

                m = 1;
                let mut b = (p.0 + m * diff.0, p.1 + m * diff.1);

                while 0 <= b.0
                    && b.0 < lines[0].len() as i32
                    && 0 <= b.1
                    && b.1 < lines.len() as i32
                {
                    m += 1;
                    if !all_antennas.contains(&b) {
                        antinodes.insert(b);
                    }
                    b = (p.0 + m * diff.0, p.1 + m * diff.1);
                }
            }
        }
    }

    println!(
        "Unique locations (2): {}",
        antinodes.len() + all_antennas.len()
    );
}
