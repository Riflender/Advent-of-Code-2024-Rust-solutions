use std::collections::HashMap;
use std::error::Error;
use crate::utils::read_file;

fn blinks(nb_blinks: usize) -> Result<u64, Box<dyn Error>> {
    let mut stones = read_file("src/inputs/input_11.txt")?
        .split(" ")
        .map(|x| (x.parse::<u64>().unwrap(), 1))
        .collect::<HashMap<u64, u64>>();

    for _ in 0..nb_blinks {
        let mut new_stones = HashMap::new();

        for stone in stones.clone() {
            if stone.0  == 0 { new_stones.entry(1).and_modify(|e| *e += stone.1).or_insert(stone.1); }
            else if stone.0.to_string().len() % 2 == 0 {
                let s = stone.0.to_string();
                new_stones.entry(s.as_str()[s.len() / 2..].parse()?).and_modify(|e| *e += stones[&stone.0]).or_insert(stones[&stone.0]);
                new_stones.entry(s.as_str()[..s.len() / 2].parse()?).and_modify(|e| *e += stones[&stone.0]).or_insert(stones[&stone.0]);
            }
            else { new_stones.entry(stone.0 * 2024).and_modify(|e| *e += stone.1).or_insert(stone.1); }
        }

        stones = new_stones;
    }

    Ok(stones.values().sum())
}

#[allow(dead_code)]
pub fn part_1() -> Result<u64, Box<dyn Error>> {
    blinks(25)
}

#[allow(dead_code)]
pub fn part_2() -> Result<u64, Box<dyn Error>> {
    blinks(75)
}