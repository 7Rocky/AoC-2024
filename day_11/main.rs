use std::fs;

use std::collections::HashMap;

struct LinkedListNode {
    value: u64,
    next: Option<Box<LinkedListNode>>,
}

struct LinkedList {
    head: Option<Box<LinkedListNode>>,
}

impl LinkedListNode {
    fn new(value: u64, next: Option<Box<LinkedListNode>>) -> LinkedListNode {
        LinkedListNode { value, next }
    }
}

impl LinkedList {
    fn new() -> LinkedList {
        LinkedList { head: None }
    }

    fn prepend(&mut self, value: u64) {
        self.head = Some(Box::new(LinkedListNode::new(value, self.head.take())));
    }

    fn len(&mut self) -> usize {
        let mut length = 0;
        let mut current_node = self.head.as_deref_mut();

        while let Some(node) = current_node {
            length += 1;
            current_node = node.next.as_deref_mut();
        }

        length
    }
}

impl Drop for LinkedList {
    fn drop(&mut self) {
        while self.head.is_some() {
            let mut el = self.head.take().unwrap();
            self.head = el.next.take();
        }
    }
}

fn split_number(s: &str) -> (u64, u64) {
    let l = s.len();

    (
        s.chars()
            .take(l / 2)
            .enumerate()
            .map(|(i, c)| u64::pow(10, (l / 2 - 1 - i) as u32) * (c.to_digit(10).unwrap() as u64))
            .sum(),
        s.chars()
            .skip(l / 2)
            .enumerate()
            .map(|(i, c)| u64::pow(10, (l / 2 - 1 - i) as u32) * (c.to_digit(10).unwrap() as u64))
            .sum(),
    )
}

fn blink(stones: &mut LinkedList) {
    let mut current_node = stones.head.as_deref_mut();

    while let Some(node) = current_node {
        let s = format!("{}", node.value);

        if node.value == 0 {
            node.value = 1;
            current_node = node.next.as_deref_mut();
        } else if s.len() % 2 == 0 {
            let (a, b) = split_number(&s);
            node.value = a;
            node.next = Some(Box::new(LinkedListNode::new(b, node.next.take())));
            current_node = node.next.as_deref_mut().and_then(|n| n.next.as_deref_mut());
        } else {
            node.value *= 2024;
            current_node = node.next.as_deref_mut();
        }
    }
}

fn count_stones(stone: u64, num_blinks: i32, cache: &mut HashMap<(u64, i32), u64>) -> u64 {
    if cache.contains_key(&(stone, num_blinks)) {
        return *cache.get(&(stone, num_blinks)).unwrap();
    }

    if num_blinks == 0 {
        cache.insert((stone, 0), 1);
        return 1;
    }

    if stone == 0 {
        let c = count_stones(1, num_blinks - 1, cache);
        cache.insert((0, num_blinks), c);
        return c;
    }

    let s = format!("{}", stone);

    if s.len() % 2 == 1 {
        let c = count_stones(stone * 2024, num_blinks - 1, cache);
        cache.insert((stone, num_blinks), c);
        return c;
    }

    let (a, b) = split_number(&s);
    let c = count_stones(a, num_blinks - 1, cache) + count_stones(b, num_blinks - 1, cache);
    cache.insert((stone, num_blinks), c);
    c
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let numbers: Vec<u64> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid integer"))
        .collect();

    let mut stones = Box::new(LinkedList::new());

    numbers.iter().rev().for_each(|&n| stones.prepend(n));
    (0..25).for_each(|_| blink(&mut stones));
    println!("Stones after blinking 25 times (1): {}", stones.len());

    println!(
        "Stones after blinking 75 times (2): {}",
        numbers
            .iter()
            .map(|&n| count_stones(n, 75, &mut HashMap::new()))
            .sum::<u64>()
    );
}
