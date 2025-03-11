use std::fs;

use std::collections::HashSet;

fn distance(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let map: Vec<Vec<char>> = input.trim().lines().map(|r| r.chars().collect()).collect();

    let mut regions: Vec<(char, HashSet<(i32, i32)>)> = Vec::new();

    let mut all_positions = (0..map.len())
        .flat_map(|j| (0..map[j].len()).map(move |i| (i as i32, j as i32)))
        .collect::<HashSet<(i32, i32)>>();

    while all_positions.len() > 0 {
        let positions = all_positions.clone();
        let mut found = false;

        for (i, j) in positions {
            for index in 0..regions.len() {
                if regions[index].0 == map[j as usize][i as usize] {
                    if regions[index]
                        .1
                        .iter()
                        .any(|&(x, y)| distance((i, j), (x, y)) == 1)
                    {
                        regions[index].1.insert((i, j));
                        all_positions.remove(&(i, j));
                        found = true;
                    }
                }
            }
        }

        if !found {
            let (i, j) = all_positions.iter().next().unwrap();
            regions.push((map[*j as usize][*i as usize], HashSet::from([(*i, *j)])));
            all_positions.remove(&(*i, *j));
        }
    }

    let mut price = (0..map.len())
        .map(|j| {
            (0..map[j].len())
                .map(|i| {
                    let mut s = 0;

                    if i == 0 || (i > 0 && map[j][i - 1] != map[j][i]) {
                        s += 1
                    }

                    if i == map[j].len() - 1 || (i < map[j].len() - 1 && map[j][i + 1] != map[j][i])
                    {
                        s += 1
                    }

                    if j == 0 || (j > 0 && map[j - 1][i] != map[j][i]) {
                        s += 1
                    }

                    if j == map.len() - 1 || (j < map.len() - 1 && map[j + 1][i] != map[j][i]) {
                        s += 1
                    }

                    regions
                        .iter()
                        .filter(|(c, v)| *c == map[j][i] && v.contains(&(i as i32, j as i32)))
                        .map(|(_, v)| (v.len() as i32) * s)
                        .sum::<i32>()
                })
                .sum::<i32>()
        })
        .sum::<i32>();

    println!("Price (1): {}", price);

    price = regions
        .iter()
        .map(|(_, region)| {
            (region.len() as i32)
                * region
                    .iter()
                    .map(|&(x, y)| {
                        vec![(1, 1), (1, -1), (-1, 1), (-1, -1)]
                            .iter()
                            .map(|&(a, b)| {
                                if !region.contains(&(x + a, y)) && !region.contains(&(x, y + b)) {
                                    1
                                } else if region.contains(&(x + a, y))
                                    && region.contains(&(x, y + b))
                                    && !region.contains(&(x + a, y + b))
                                {
                                    1
                                } else {
                                    0
                                }
                            })
                            .sum::<i32>()
                    })
                    .sum::<i32>()
        })
        .sum::<i32>();

    println!("Price (2): {}", price);
}
