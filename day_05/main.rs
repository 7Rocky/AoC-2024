use std::fs;

fn is_good(list: &Vec<i32>, orders: &Vec<Vec<i32>>) -> bool {
    orders.iter().all(|order| {
        if list.contains(&order[0]) && list.contains(&order[1]) {
            let a = list.iter().position(|r| *r == order[0]).unwrap();
            let b = list.iter().position(|r| *r == order[1]).unwrap();
            a < b
        } else {
            true
        }
    })
}

fn find_middle_incorrect(mut list: Vec<i32>, orders: &Vec<Vec<i32>>) -> i32 {
    while !is_good(&list, orders) {
        for order in orders {
            if list.contains(&order[0]) && list.contains(&order[1]) {
                let a = list.iter().position(|r| *r == order[0]).unwrap();
                let b = list.iter().position(|r| *r == order[1]).unwrap();

                if b < a {
                    let t = list[b];
                    list[b] = list[a];
                    list[a] = t;
                    break;
                }
            }
        }
    }

    list[list.len() / 2]
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let lines: Vec<_> = input.trim().lines().collect();
    let mut blank = false;
    let mut orders: Vec<Vec<i32>> = vec![];
    let mut lists: Vec<Vec<i32>> = vec![];

    for line in lines {
        if line == "" {
            blank = true;
            continue;
        }

        if blank {
            lists.push(
                line.split(",")
                    .map(|s| s.parse().expect("Invalid integer"))
                    .collect(),
            );
        } else {
            orders.push(
                line.split("|")
                    .map(|s| s.parse().expect("Invalid integer"))
                    .collect(),
            );
        }
    }

    let mut correct = 0;
    let mut incorrect = 0;

    for list in lists {
        if is_good(&list, &orders) {
            correct += list[list.len() / 2];
        } else {
            incorrect += find_middle_incorrect(list, &orders);
        }
    }

    println!("Middle page numbers of correct pages (1): {correct}");
    println!("Middle page numbers of incorrect pages (2): {incorrect}");
}
