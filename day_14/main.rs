use std::collections::HashSet;
use std::fs;
use std::str::FromStr;

const HEIGHT: i32 = 103;
const WIDTH: i32 = 101;

#[derive(Debug)]
struct Robot {
    px: i32,
    py: i32,
    vx: i32,
    vy: i32,
}

impl FromStr for Robot {
    type Err = String;

    fn from_str(raw_robot: &str) -> Result<Self, Self::Err> {
        let p: &str = raw_robot.split(" ").nth(0).unwrap();
        let v: &str = raw_robot.split(" ").nth(1).unwrap();
        let px: i32 = p.split("=").nth(1).unwrap().split(",").nth(0).unwrap().parse().unwrap();
        let py: i32 = p.split("=").nth(1).unwrap().split(",").nth(1).unwrap().parse().unwrap();
        let vx: i32 = v.split("=").nth(1).unwrap().split(",").nth(0).unwrap().parse().unwrap();
        let vy: i32 = v.split("=").nth(1).unwrap().split(",").nth(1).unwrap().parse().unwrap();

        Ok(Robot {
            px: px,
            py: py,
            vx: vx,
            vy: vy,
        })
    }
}

impl Robot {
    fn movement(&mut self) {
        self.px = (self.px + self.vx + WIDTH) % WIDTH;
        self.py = (self.py + self.vy + HEIGHT) % HEIGHT;
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let mut robots: Vec<Robot> = input
        .lines()
        .map(|r| r.parse())
        .collect::<Result<_, _>>()
        .unwrap();


    (0..100).for_each(|_| robots.iter_mut().for_each(|r| r.movement()));

    let mut quadrants = vec![0, 0, 0, 0];

    robots.iter().for_each(|r| {
        if r.px > WIDTH / 2 && r.py < HEIGHT / 2 {
            quadrants[0] += 1
        } else if r.px < WIDTH / 2 && r.py < HEIGHT / 2 {
            quadrants[1] += 1
        } else if r.px < WIDTH / 2 && r.py > HEIGHT / 2 {
            quadrants[2] += 1
        } else if r.px > WIDTH / 2 && r.py > HEIGHT / 2 {
            quadrants[3] += 1
        }
    });

    println!("Safety factor (1): {}", quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3]);

    let mut positions: HashSet<(i32, i32)> = HashSet::new();

    for i in 100 + 1..=(WIDTH * HEIGHT) {
        positions.clear();

        robots.iter_mut().for_each(|r| {
            r.movement();
            positions.insert((r.px, r.py));
        });

        if positions.len() == robots.len() {
            println!("Time elapsed (2): {}", i);
            break
        }
    }
}
