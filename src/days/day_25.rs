use std::error::Error;
use crate::utils::{iterate_on_lines_batch, read_file};

fn parse_batch(batch: &[String]) -> Result<Vec<Vec<char>>, Box<dyn Error>> {
    Ok(batch[..7].into_iter().map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>())
}

#[allow(dead_code)]
pub fn part_1() -> Result<usize, Box<dyn Error>> {
    let lines = iterate_on_lines_batch("src/inputs/input_25.txt", 8, &mut parse_batch)?;
    // let locks = vec![(0, 5, 3, 4, 3), (1, 2, 0, 5, 3)];
    // let keys = vec![(5, 0, 2, 1, 3), (4, 3, 4, 0, 2), (3, 0, 2, 0, 1)];

    let mut locks = Vec::new();
    let mut keys = Vec::new();
    for schematic in lines {
        let mut heights = (0_u8, 0_u8, 0_u8, 0_u8, 0_u8);
        for row in &schematic[1..schematic.len() - 1] {
            if row[0] == '#' { heights.0 += 1; }
            if row[1] == '#' { heights.1 += 1; }
            if row[2] == '#' { heights.2 += 1; }
            if row[3] == '#' { heights.3 += 1; }
            if row[4] == '#' { heights.4 += 1; }
        }

        if schematic[0][0] == '#' { locks.push(heights) }
        else { keys.push(heights) }
    }

    let mut res = 0;
    for lock in locks {
        for key in keys.clone() {
            if lock.0 + key.0 < 6 &&
                lock.1 + key.1 < 6 &&
                lock.2 + key.2 < 6 &&
                lock.3 + key.3 < 6 &&
                lock.4 + key.4 < 6
            {
                res += 1;
            }
        }
    }

    Ok(res)
}

#[allow(dead_code)]
pub fn part_2() -> Result<usize, Box<dyn Error>> {
    let _lines = read_file("src/inputs/input_25.txt")?;



    Ok(0)
}