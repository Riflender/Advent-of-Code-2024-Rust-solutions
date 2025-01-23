use std::error::Error;
use crate::utils::iterate_on_lines;

fn parse_line(line: &str) -> Result<Vec<i8>, Box<dyn Error>> {
    Ok(line.split(" ").map(|x| x.parse::<i8>().unwrap()).collect::<Vec<i8>>())
}

fn is_report_safe(report: Vec<i8>) -> bool {
    let mut previous: Option<i8> = None;

    for x in report.windows(2) {
        let current = x[1] - x[0];
        if current.abs() < 1 || current.abs() > 3 {
            return false
        } else if let Some(p) = previous {
            if current.is_positive() != p.is_positive() {
                return false
            }
        }

        previous = Some(current)
    }

    true
}

#[allow(dead_code)]
pub fn part_1() -> Result<u32, Box<dyn Error>> {
    let lines = iterate_on_lines("src/inputs/input_02.txt", parse_line)?;
    /*
    let lines = vec![
        vec![7, 6, 4, 2, 1],
        vec![1, 2, 7, 8, 9],
        vec![9, 7, 6, 2, 1],
        vec![1, 3, 2, 4, 5],
        vec![8, 6, 4, 4, 1],
        vec![1, 3, 6, 7, 9]
    ];
    */

    let mut safe = 0;
    for line in lines { if is_report_safe(line) { safe += 1; } }

    Ok(safe)
}

#[allow(dead_code)]
pub fn part_2() -> Result<u32, Box<dyn Error>> {
    let lines = iterate_on_lines("src/inputs/input_02.txt", parse_line)?;
    /*
    let lines = vec![
        vec![7, 6, 4, 2, 1],
        vec![1, 2, 7, 8, 9],
        vec![9, 7, 6, 2, 1],
        vec![1, 3, 2, 4, 5],
        vec![8, 6, 4, 4, 1],
        vec![1, 3, 6, 7, 9]
    ];
    */

    let mut safe = 0;
    for line in lines {
        if is_report_safe(line.clone()) { safe += 1; }
        else {
            for i in 0..line.len() {
                let mut tmp = line.clone();
                tmp.remove(i);
                if is_report_safe(tmp) {
                    safe += 1;
                    break;
                }
            }
        }
    }

    Ok(safe)
}
