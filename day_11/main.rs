use std::fs;

use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct LinkedListNode<T> {
    value: T,
    next: Option<Box<LinkedListNode<T>>>,
}

#[derive(Debug, PartialEq)]
struct LinkedList<T> {
    head: Option<Box<LinkedListNode<T>>>,
    tail: Option<*mut LinkedListNode<T>>,
}

impl<T> LinkedListNode<T> {
    fn new(value: T, next: Option<Box<LinkedListNode<T>>>) -> LinkedListNode<T> {
        LinkedListNode { value, next }
    }
}

impl<T: PartialEq> LinkedList<T> {
    fn new() -> LinkedList<T> {
        LinkedList {
            head: None,
            tail: None,
        }
    }

    fn append(&mut self, value: T) {
        let new_node = Box::new(LinkedListNode::new(value, None));

        unsafe {
            let raw_node: *mut _ = Box::into_raw(new_node);

            if self.head.is_none() {
                self.head = Some(Box::from_raw(raw_node));
            } else if let Some(tail) = self.tail {
                (*tail).next = Some(Box::from_raw(raw_node));
            }

            self.tail = Some(raw_node);
        }
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

fn blink(stones: &mut LinkedList<u64>) {
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

    let mut stones = Box::new(LinkedList::<u64>::new());

    numbers.iter().for_each(|&n| stones.append(n));
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
