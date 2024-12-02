use std::fs;

fn is_safe(diffs: &Vec<i32>) -> bool {
    diffs.iter().all(|&x| (1..=3).contains(&x)) || diffs.iter().all(|&x| (-3..=-1).contains(&x))
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let all_diffs: Vec<_> = input
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse().expect("Invalid integer"))
                .collect::<Vec<i32>>()
        })
        .map(|numbers| {
            numbers
                .windows(2)
                .map(|pair| pair[0] - pair[1])
                .collect::<Vec<_>>()
        })
        .collect();

    let mut s = all_diffs.iter().filter(|diffs| is_safe(&diffs)).count();

    println!("Number of safe reports (1): {s}");

    s += all_diffs
        .iter()
        .filter(|diffs| !is_safe(&diffs))
        .filter(|diffs| {
            (0..=diffs.len()).any(|i| {
                let mut new_diffs: Vec<i32> = vec![0; diffs.len() - 1];

                if i == diffs.len() {
                    new_diffs = diffs.iter().skip(1).map(|x| *x).collect();
                } else {
                    let mut j = 0;
                    let mut k = 0;

                    while j < diffs.len() - 1 {
                        new_diffs[j] = diffs[j + k];

                        if j == i {
                            k = 1;
                            new_diffs[j] += diffs[j + k];
                        }

                        j += 1;
                    }
                }

                is_safe(&new_diffs)
            })
        })
        .count();

    println!("Number of safe reports (2): {s}");
}
