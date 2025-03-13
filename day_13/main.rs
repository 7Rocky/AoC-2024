use std::fs;
use std::str::FromStr;

struct Machine {
    ax: i64,
    ay: i64,
    bx: i64,
    by: i64,
    px: i64,
    py: i64,
}

impl FromStr for Machine {
    type Err = String;

    fn from_str(raw_machine: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = raw_machine.lines().collect();

        fn extract_value(s: &str, prefix: &str) -> i64 {
            s.split_once(prefix)
                .and_then(|(_, num)| num.split(',').next())
                .unwrap()
                .trim()
                .parse::<i64>()
                .unwrap()
        }

        Ok(Machine {
            ax: extract_value(parts[0], "X+"),
            ay: extract_value(parts[0], "Y+"),
            bx: extract_value(parts[1], "X+"),
            by: extract_value(parts[1], "Y+"),
            px: extract_value(parts[2], "X="),
            py: extract_value(parts[2], "Y="),
        })
    }
}

impl Machine {
    fn solution(&self) -> (i64, i64) {
        let d = self.ay * self.bx - self.ax * self.by;
        let na = self.bx * self.py - self.px * self.by;
        let nb = self.ay * self.px - self.ax * self.py;

        if d != 0 && na % d == 0 && nb % d == 0 {
            (na / d, nb / d)
        } else {
            (0, 0)
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let mut machines: Vec<Machine> = input
        .trim()
        .split("\n\n")
        .map(|m| m.parse())
        .collect::<Result<_, _>>()
        .unwrap();

    let tokens: i64 = machines
        .iter()
        .map(|m| m.solution())
        .map(|(a, b)| 3 * a + b)
        .sum();

    println!("Tokens (1): {}", tokens);

    let tokens: i64 = machines
        .iter_mut()
        .map(|m| {
            m.px += 10000000000000;
            m.py += 10000000000000;
            m.solution()
        })
        .map(|(a, b)| 3 * a + b)
        .sum();

    println!("Tokens (2): {}", tokens);
}
