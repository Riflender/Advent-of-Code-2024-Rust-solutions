use std::error::Error;
use crate::utils::iterate_on_lines;

fn parse_line(line: &str) -> Result<Vec<i8>, Box<dyn Error>> {
    Ok(line.split(" ").map(|x| x.parse::<i8>().unwrap()).collect::<Vec<i8>>())
}

#[allow(dead_code)]
pub fn part_1() -> Result<u32, Box<dyn Error>> {
    let lines = iterate_on_lines("src/inputs/input_02.txt", &mut parse_line)?;
    let mut safe = 0;

    for line in lines {
        let mut previous: Option<i8> = None;

        for x in line.windows(2) {
            let current = x[1] - x[0];
            if current.abs() < 1 || current.abs() > 3 {
                previous = None;
                break;
            } else if let Some(p) = previous {
                if current.is_positive() != p.is_positive() {
                    previous = None;
                    break;
                }
            }

            previous = Some(current)
        }

        if let Some(_) = previous { safe += 1; };
    }

    Ok(safe)
}

#[allow(dead_code)]
struct Dampener {
    activated: bool,
    used: bool,
    buffer: i8,
}

#[allow(dead_code)]
pub fn part_2() -> Result<u32, Box<dyn Error>> {


    Ok(0)
}
