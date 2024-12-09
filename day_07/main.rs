use std::fs;

use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let mut hm = HashMap::new();

    input.trim().lines().for_each(|line| {
        let parts: Vec<_> = line.split(": ").collect();
        let key: i64 = parts[0].parse().expect("Invalid integer");
        let value: Vec<i64> = parts[1]
            .split_whitespace()
            .map(|i| i.parse().expect("Invalid integer"))
            .collect();

        hm.insert(key, value);
    });

    let mut calibration_results = [0, 0];

    for (key, value) in hm.into_iter() {
        for i in 0..(1 << value.len()) {
            let mut n = i;
            let mut s = value[0];

            for j in 1..value.len() {
                if n & 1 == 0 {
                    s += value[j];
                } else {
                    s *= value[j];
                }

                n >>= 1;
            }

            if s == key {
                calibration_results[0] += key;
                break;
            }
        }

        for i in 0..i64::pow(3, value.len() as u32 - 1) {
            let mut n = i;
            let mut s = value[0];

            for j in 1..value.len() {
                if n % 3 == 0 {
                    s += value[j];
                } else if n % 3 == 1 {
                    s *= value[j];
                } else {
                    s = format!("{s}{}", value[j]).parse().unwrap();
                }

                n /= 3;
            }

            if s == key {
                calibration_results[1] += key;
                break;
            }
        }
    }

    println!("Calibration result (1): {}", calibration_results[0]);
    println!("Calibration result (2): {}", calibration_results[1]);
}
