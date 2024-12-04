use std::fs;

fn find_xmas(s: &str) -> i32 {
    ((2 * s.len() - s.replace("XMAS", "").len() - s.replace("SAMX", "").len()) / 4) as i32
}

fn find_x_mas(lines: Vec<&str>) -> i32 {
    (1..lines.len() - 1)
        .map(|i| {
            (1..lines[0].len() - 1)
                .filter(|&j| {
                    let m = lines[i].chars().nth(j).unwrap();
                    let ul = lines[i - 1].chars().nth(j - 1).unwrap();
                    let ur = lines[i - 1].chars().nth(j + 1).unwrap();
                    let dl = lines[i + 1].chars().nth(j - 1).unwrap();
                    let dr = lines[i + 1].chars().nth(j + 1).unwrap();

                    m == 'A'
                        && ((ul == 'M' && ur == 'M' && dl == 'S' && dr == 'S')
                            || (ul == 'M' && ur == 'S' && dl == 'M' && dr == 'S')
                            || (ul == 'S' && ur == 'M' && dl == 'S' && dr == 'M')
                            || (ul == 'S' && ur == 'S' && dl == 'M' && dr == 'M'))
                })
                .count() as i32
        })
        .sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let lines: Vec<_> = input.trim().split("\n").collect();

    let vertical: Vec<_> = (0..lines[0].len())
        .map(|i| {
            lines
                .iter()
                .map(|line| line.chars().nth(i).unwrap())
                .collect::<String>()
        })
        .collect();

    let mut diagonal: Vec<_> = vec![];
    let x = lines[0].len();
    let y = lines.len();

    for i in 1..y {
        diagonal.push(
            (0..y - (i as usize))
                .map(|k| lines[(i as usize) + k].chars().nth(k).unwrap())
                .collect::<String>(),
        );

        diagonal.push(
            (0..(i as usize))
                .map(|k| lines[(i as usize) - 1 - k].chars().nth(k).unwrap())
                .collect::<String>(),
        );
    }

    for j in (0..x).rev() {
        diagonal.push(
            (0..x - (j as usize))
                .map(|k| lines[k].chars().nth((j as usize) + k).unwrap())
                .collect::<String>(),
        );

        diagonal.push(
            (0..x - (j as usize))
                .map(|k| lines[y - 1 - k].chars().nth((j as usize) + k).unwrap())
                .collect::<String>(),
        );
    }

    let s = lines.iter().map(|line| find_xmas(line)).sum::<i32>()
        + vertical.iter().map(|line| find_xmas(line)).sum::<i32>()
        + diagonal.iter().map(|line| find_xmas(line)).sum::<i32>();

    println!("Number of XMAS (1): {s}");

    println!("Number of X-MAS (2): {}", find_x_mas(lines));
}
