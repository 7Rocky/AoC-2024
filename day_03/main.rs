use std::fs;

fn find_mul(s: &str) -> (i32, usize) {
    let start = match s.find("mul(") {
        Some(start) => start,
        None => return (0, 4),
    };

    let end = match s.get(start + 4..).unwrap().find(")") {
        Some(end) => end,
        None => return (0, 0),
    };

    let middle: String = s.chars().skip(start + 4).take(end).collect::<String>();

    if middle.chars().all(|c| "0123456789,".contains(c)) {
        let v: Vec<i32> = middle
            .split(",")
            .map(|x| x.parse().expect("Invalid integer"))
            .collect();
        (v[0] * v[1], start + end + 5)
    } else {
        (0, start + 4)
    }
}

fn program(line: &str) -> i32 {
    let (mut p, mut end): (i32, usize) = find_mul(line);
    let mut res: i32 = p;
    let mut next: usize = end;

    while next < line.len() {
        (p, end) = find_mul(line.get(next..).unwrap());
        res += p;
        next += end;
    }

    res
}

fn main() {
    let line = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file")
        .replace("\n", "");

    let mut s = program(&line as &str);

    println!("Program output (1): {s}");

    s = line
        .split("do()")
        .map(|p| p.split("don't()").next().unwrap())
        .map(|line| program(&line as &str))
        .sum();

    println!("Program output (2): {s}");
}
