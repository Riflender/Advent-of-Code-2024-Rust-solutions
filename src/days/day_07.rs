use std::error::Error;
use crate::utils::iterate_on_lines;

fn parse_line(line: &str) -> Result<(u64, Vec<u16>), Box<dyn Error>> {
    let mut parts = line.split(": ");
    let first: u64 = parts.next().ok_or("Pas assez de parties dans la chaîne")?.parse()?;
    let second = parts.next()
        .ok_or("Pas assez de parties dans la chaîne")?.
        split(" ")
        .map(|x| x.parse::<u16>().unwrap())
        .collect::<Vec<u16>>();

    Ok((first, second))
}

#[allow(dead_code)]
pub fn part_1() -> Result<u64, Box<dyn Error>> {
    let lines = iterate_on_lines("src/inputs/input_07.txt", &parse_line)?;
    let mut res = 0;

    for line in lines {
        let mut tmp_res = 0;
        let first_number = line.1[0] as u64;

        let nb_operators = line.1.len() - 1;
        for i in 0..2_u16.pow(nb_operators as u32) {
            let bi = format!("{i:0nb_operators$b}")
                .chars()
                .map(|c| c.to_digit(2).unwrap() != 0)
                .collect::<Vec<bool>>();
            tmp_res = first_number;

            for (n, o) in line.1.iter().skip(1).zip(bi.clone()) {
                match o {
                    true => tmp_res += *n as u64,
                    false => tmp_res *= *n as u64,
                }
            }

            if line.0 == tmp_res { break; }
        }

        if line.0 == tmp_res { res += line.0; }
    }

    Ok(res)
}

fn to_ternary(value: u32, nb_operators: usize) -> Vec<u32> {
    if value == 0 { return vec![0; nb_operators]; }

    let mut tmp = value;
    let mut vec = Vec::new();

    while tmp != 0 {
        vec.push(tmp % 3);
        tmp /= 3;
    }

    let mut res = vec![0; nb_operators - vec.len()];
    res.append(&mut vec.into_iter().rev().collect::<Vec<u32>>());
    res
}

fn concatenate(x: u64, y: u64) -> u64 {
    let mut pow = 10;
    while y >= pow { pow *= 10; }
    x * pow + y
}

#[allow(dead_code)]
pub fn part_2() -> Result<u64, Box<dyn Error>> {
    let lines = iterate_on_lines("src/inputs/input_07.txt", &parse_line)?;
    let mut res = 0;

    for line in lines.clone() {
        let mut tmp_res = 0;
        let first_number = line.1[0] as u64;

        let nb_operators = line.1.len() - 1;
        for i in 0..3_u32.pow(nb_operators as u32) {
            let ti = to_ternary(i, nb_operators);
            tmp_res = first_number;

            for (n, o) in line.1.iter().skip(1).zip(ti.clone()) {
                match o {
                    0 => tmp_res += *n as u64,
                    1 => tmp_res *= *n as u64,
                    2 => tmp_res = concatenate(tmp_res, *n as u64),
                    _ => return Err(Box::from("Mauvaise conversion")),
                }
            }

            if line.0 == tmp_res { break; }
        }

        if line.0 == tmp_res { res += line.0; }
    }

    Ok(res)
}
