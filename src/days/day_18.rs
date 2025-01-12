use std::error::Error;
use crate::utils::{iterate_on_lines, read_file};

fn parse_line(line: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let mut parts = line.split(",");
    let first = parts.next().ok_or("Pas assez de parties dans la chaîne")?.parse()?;
    let second = parts.next().ok_or("Pas assez de parties dans la chaîne")?.parse()?;

    Ok((first, second))
}

#[allow(dead_code)]
pub fn part_1() -> Result<usize, Box<dyn Error>> {
    let lines = iterate_on_lines("src/inputs/input_18.txt", parse_line)?;
    let n = 71;
    /*
    let lines = vec![
        (5, 4),
        (4, 2),
        (4, 5),
        (3, 0),
        (2, 1),
        (6, 3),
        (2, 4),
        (1, 5),
        (0, 6),
        (3, 3),
        (2, 6),
        (5, 1),
        (1, 2),
        (5, 5),
        (2, 5),
        (6, 5),
        (1, 4),
        (0, 4),
        (6, 4),
        (1, 1),
        (6, 1),
        (1, 0),
        (0, 5),
        (1, 6),
        (2, 0)
    ];
    let n = 7;
    */

    let mut grid = vec![vec!['.'; n]; n];
    for (x, y) in lines { grid[y][x] = '#'; }

    Ok(0)
}

#[allow(dead_code)]
pub fn part_2() -> Result<usize, Box<dyn Error>> {
    let _lines = read_file("src/inputs/input_18.txt")?;



    Ok(0)
}