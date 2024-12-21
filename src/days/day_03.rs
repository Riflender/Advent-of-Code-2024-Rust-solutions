use std::error::Error;
use regex::Regex;
use crate::utils::read_file;

#[allow(dead_code)]
pub fn part_1() -> Result<u32, Box<dyn Error>> {
    let input = read_file("src/inputs/input_03.txt")?;
    let int_regex = Regex::new(r"\d+")?;

    let res: u32 = Regex::new(r"mul\(\d+,\d+\)")?
        .find_iter(&*input)
        .map(|m| int_regex
            .find_iter(m.as_str())
            .map(|m| m.as_str().parse::<u16>().unwrap())
            .collect::<Vec<u16>>())
        .collect::<Vec<Vec<u16>>>()
        .iter()
        .map(|x| x[0] as u32 * x[1] as u32)
        .collect::<Vec<u32>>()
        .into_iter()
        .sum();

    Ok(res)
}

#[allow(dead_code)]
pub fn part_2() -> Result<u32, Box<dyn Error>> {
    let input = read_file("src/inputs/input_03.txt")?;
    let int_regex = Regex::new(r"\d+")?;

    let instructions = Regex::new(r"(mul\(\d+,\d+\))|(do(n\'t)?\(\))")?
        .find_iter(&*input)
        .map(|m| m.as_str())
        .collect::<Vec<&str>>();

    let mut dont = false;
    let mut res: u32 = 0;
    for instruction in instructions {
        match instruction {
            "don't()" => dont = true,
            "do()" => dont = false,
            mul if !dont => {
                let tmp = int_regex.find_iter(mul)
                    .map(|m| m.as_str().parse::<u16>().unwrap())
                    .collect::<Vec<u16>>();
                res += tmp[0] as u32 * tmp[1] as u32;
            }
            _ => continue,
        }
    }

    Ok(res)
}
