use std::error::Error;
use crate::utils::{file_to_chars, Direction};

#[allow(dead_code)]
pub fn part_1() -> Result<usize, Box<dyn Error>> {
    let mut lines = file_to_chars("src/inputs/input_06.txt")?;

    let n = 130;
    let mut guard = (80, 79);
    let mut direction = Direction::Up;
    let mut forward: (isize, isize) = (-1, 0);

    let mut next = (guard.0 + forward.0, guard.1 + forward.1);

    while 0 <= next.0 && next.0 < n as isize && 0 <= next.1 && next.1 < n as isize {
        lines[guard.0 as usize][guard.1 as usize] = 'X';

        if let '#' = lines[next.0 as usize][next.1 as usize] { forward = direction.next(); }
        else { guard = next; }

        next = (guard.0 + forward.0, guard.1 + forward.1);
    }

    let mut res = 0;
    for i in 0..n {
        for j in 0..n {
            if let 'X' = lines[i][j] { res += 1; }
        }
    }

    Ok(res + 1)
}

#[allow(dead_code)]
pub fn part_2() -> Result<usize, Box<dyn Error>> {
    let _lines = file_to_chars("src/inputs/input_06.txt")?;



    Ok(0)
}
