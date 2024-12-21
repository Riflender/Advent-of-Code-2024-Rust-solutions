use std::error::Error;
use crate::utils::file_to_bytes;

fn count_xmas(line: &Vec<u8>, pattern: &str) -> Result<usize, Box<dyn Error>> {
    Ok(
        String::from_utf8(line.clone())?
            .match_indices(pattern)
            .collect::<Vec<_>>()
            .len() +
        String::from_utf8(line.iter().copied().rev().collect::<Vec<u8>>())?
            .match_indices(pattern)
            .collect::<Vec<_>>()
            .len()
    )
}

#[allow(dead_code)]
pub fn part_1() -> Result<usize, Box<dyn Error>> {
    let lines = file_to_bytes("src/inputs/input_04.txt")?;
    let mut xmas = 0;

    for line in &lines {
        xmas += count_xmas(line, "XMAS")?;
    }
    for col in 0..lines[0].len() {
        let mut line = vec![];
        for row in &lines {
            line.push(row[col]);
        }
        xmas += count_xmas(&line, "XMAS")?;
    }

    let height = lines.len();
    let width = lines[0].len();
    for diag in 0..(height + width - 1) {
        let (mut i, mut j) =
            if diag >= height { (0, diag - height + 1) }
            else { (height - diag - 1, 0) };
        let mut line = vec![];
        while i < height && j < width {
            line.push(lines[i][j]);
            i += 1;
            j += 1;
        }
        xmas += count_xmas(&line, "XMAS")?;

        let (mut i, mut j) =
            if diag >= height { (0, ((height + width - 1) - diag)) }
            else { (height - diag - 1, width) };
        let mut line = vec![];
        while i < height && j >= 1 {
            line.push(lines[i][j - 1]);
            i += 1;
            j -= 1;
        }
        xmas += count_xmas(&line, "XMAS")?;
    }

    Ok(xmas)
}

#[allow(dead_code)]
pub fn part_2() -> Result<usize, Box<dyn Error>> {
    let lines = file_to_bytes("src/inputs/input_04.txt")?;
    let mut xmas = 0;

    let height = lines.len();
    let width = lines[0].len();

    for i in 1..(height - 1) {
        for j in 1..(width - 1) {
            if lines[i][j] != 65 { continue; }

            let nb_mas =
                (String::from_utf8(vec![lines[i - 1][j - 1], lines[i][j], lines[i + 1][j + 1]])? == "MAS") as usize +
                (String::from_utf8(vec![lines[i + 1][j - 1], lines[i][j], lines[i - 1][j + 1]])? == "MAS") as usize +
                (String::from_utf8(vec![lines[i - 1][j + 1], lines[i][j], lines[i + 1][j - 1]])? == "MAS") as usize +
                (String::from_utf8(vec![lines[i + 1][j + 1], lines[i][j], lines[i - 1][j - 1]])? == "MAS") as usize;

            if nb_mas > 1 { xmas += 1; }
        }
    }

    Ok(xmas)
}
