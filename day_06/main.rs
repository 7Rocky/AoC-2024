use std::fs;

use std::collections::HashSet;

#[derive(Eq, Hash, PartialEq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Clone for Direction {
    fn clone(&self) -> Direction {
        match self {
            Direction::UP => Direction::UP,
            Direction::DOWN => Direction::DOWN,
            Direction::LEFT => Direction::LEFT,
            Direction::RIGHT => Direction::RIGHT,
        }
    }
}

fn run(position: &mut (i32, i32), direction: &mut Direction, map: &Vec<Vec<&u8>>) -> bool {
    match *direction {
        Direction::UP => {
            if position.1 == 0 {
                return true;
            } else if *map[position.1 as usize - 1][position.0 as usize] == b'#' {
                *direction = Direction::RIGHT;
            } else {
                position.1 -= 1;
            }
        }

        Direction::RIGHT => {
            if position.0 == map[0].len() as i32 - 1 {
                return true;
            } else if *map[position.1 as usize][position.0 as usize + 1] == b'#' {
                *direction = Direction::DOWN;
            } else {
                position.0 += 1;
            }
        }

        Direction::DOWN => {
            if position.1 == map.len() as i32 - 1 {
                return true;
            } else if *map[position.1 as usize + 1][position.0 as usize] == b'#' {
                *direction = Direction::LEFT;
            } else {
                position.1 += 1;
            }
        }

        Direction::LEFT => {
            if position.0 == 0 {
                return true;
            } else if *map[position.1 as usize][position.0 as usize - 1] == b'#' {
                *direction = Direction::UP;
            } else {
                position.0 -= 1;
            }
        }
    }

    return false;
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let mut map: Vec<Vec<&u8>> = input
        .trim()
        .lines()
        .map(|r| r.as_bytes().into_iter().collect())
        .collect();

    let mut position = (0i32, 0i32);

    for j in 0..map.len() {
        for i in 0..map[j].len() {
            if *map[j][i] == b'^' {
                position.0 = i as i32;
                position.1 = j as i32;
            }
        }
    }

    let original_position = (position.0, position.1);

    let mut direction = Direction::UP;
    let mut places = HashSet::new();

    while 0 <= position.0
        && position.0 < map[0].len() as i32
        && 0 <= position.1
        && position.1 < map.len() as i32
    {
        places.insert((position.0, position.1));

        if run(&mut position, &mut direction, &map) {
            break;
        }
    }

    println!("Distinct positions (1): {}", places.len());

    let s = (0..map.len())
        .map(|j| {
            (0..map[j].len())
                .map(|i| {
                    let mut s = 0;

                    if *map[j][i] == b'#' || *map[j][i] == b'^' {
                        return 0;
                    }

                    map[j][i] = &b'#';

                    position.0 = original_position.0;
                    position.1 = original_position.1;
                    direction = Direction::UP;

                    let mut places = HashSet::new();

                    while 0 <= position.0
                        && position.0 < map[0].len() as i32
                        && 0 <= position.1
                        && position.1 < map.len() as i32
                    {
                        if places.contains(&(position.0, position.1, direction.clone())) {
                            s += 1;
                            break;
                        }

                        places.insert((position.0, position.1, direction.clone()));

                        if run(&mut position, &mut direction, &map) {
                            break;
                        }
                    }

                    map[j][i] = &b'.';

                    return s;
                })
                .sum::<i32>()
        })
        .sum::<i32>();

    println!("Obstruction places (2): {s}");
}
