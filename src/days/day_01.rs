use std::error::Error;
use std::iter::zip;
use crate::utils::iterate_on_lines;

fn parse_line(line: &str) -> Result<(i32, i32), Box<dyn Error>> {
    let mut parts = line.split_whitespace();
    let first: i32 = parts.next().ok_or("Pas assez de parties dans la chaîne")?.parse()?;
    let second: i32 = parts.next().ok_or("Pas assez de parties dans la chaîne")?.parse()?;

    Ok((first, second))
}

#[allow(dead_code)]
pub fn part_1() -> Result<i32, Box<dyn Error>> {
    let lines = iterate_on_lines("src/inputs/input_01.txt", &parse_line)?;

    let (mut left, mut right): (Vec<i32>, Vec<i32>) = lines.into_iter().unzip();
    left.sort();
    right.sort();

    let mut res = 0;
    for (l, r) in zip(left, right) {
        res += (l - r).abs();
    }

    Ok(res)
}

#[allow(dead_code)]
pub fn part_2() -> Result<i32, Box<dyn Error>> {
    let lines = iterate_on_lines("src/inputs/input_01.txt", &parse_line)?;

    let (left, right): (Vec<i32>, Vec<i32>) = lines.into_iter().unzip();

    let mut res = 0;
    for l in left {
        res += l * right.iter().filter(|&x| *x == l).count() as i32;
    }

    Ok(res)
}
