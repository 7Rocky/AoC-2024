use std::fs;

fn get_blocks(disk_map: &str) -> (Vec<i64>, i64) {
    let mut id = 0;
    let mut blocks = Vec::<i64>::new();

    for chunk in disk_map.as_bytes().chunks(2) {
        let a = (chunk[0] as char).to_digit(10).unwrap() as usize;
        let b = (chunk[1] as char).to_digit(10).unwrap() as usize;

        blocks.extend(vec![id; a]);
        blocks.extend(vec![-1; b]);
        id += 1;
    }

    (blocks, id - 1)
}

fn checksum(blocks: &Vec<i64>) -> i64 {
    blocks
        .iter()
        .enumerate()
        .filter_map(|(i, &b)| Some(if b != -1 { (i as i64) * b } else { 0 }))
        .sum()
}

fn find_hole(blocks: &Vec<i64>, length: usize) -> usize {
    let mut i: usize = 0;
    let mut size: usize = 0;

    while i < blocks.len() {
        while i < blocks.len() && blocks[i] == -1 {
            size += 1;
            i += 1;
        }

        if size >= length {
            break;
        }

        size = 0;
        i += 1;
    }

    i - size
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let mut disk_map = input.trim().to_owned();

    if disk_map.len() % 2 == 1 {
        disk_map += "0";
    }

    let (mut blocks, mut id) = get_blocks(&disk_map);
    let original_blocks = blocks.clone();

    let mut holes: usize = blocks.iter().take_while(|&&b| b != -1).count();
    let mut disk: usize = blocks.len() - 1 - blocks.iter().rev().take_while(|&&b| b == -1).count();

    while holes < disk - 1 {
        blocks[holes] = blocks[disk];
        blocks[disk] = -1;

        holes += blocks[holes..].iter().take_while(|&&b| b != -1).count();
        disk -= blocks[..=disk].iter().rev().take_while(|&&b| b == -1).count();
    }

    println!("Checksum (1): {}", checksum(&blocks));

    blocks = original_blocks;

    disk = blocks.len() - 1 - blocks.iter().rev().take_while(|&&b| b == -1).count();
    let mut length = blocks[..=disk].iter().rev().take_while(|&&b| b == id).count();

    while id > 0 {
        let i = find_hole(&blocks, length);

        if i != 0 && i < disk {
            for j in 0..length {
                blocks[i + j] = id;
                blocks[disk - j] = -1;
            }
        }

        id -= 1;
        disk -= length;
        disk -= blocks[..=disk].iter().rev().take_while(|&&b| b != id).count();
        length = blocks[..=disk].iter().rev().take_while(|&&b| b == id).count();
    }

    println!("Checksum (2): {}", checksum(&blocks));
}
