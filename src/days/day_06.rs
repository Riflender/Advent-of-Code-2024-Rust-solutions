use std::error::Error;
use crate::utils::{file_to_chars, Direction};

const START_GUARD: (isize, isize) = (80, 79);
const START_DIRECTION: Direction = Direction::Up;

#[allow(dead_code)]
pub fn part_1() -> Result<usize, Box<dyn Error>> {
    let mut lines = file_to_chars("src/inputs/input_06.txt")?;

    let n = lines.len();
    let mut guard = START_GUARD;
    let mut direction = START_DIRECTION;
    let mut forward: (isize, isize) = (&direction).into();

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
