use std::collections::HashSet;
use std::error::Error;
use crate::utils::{file_to_chars, Direction};

fn find_guard(lines: Vec<Vec<char>>) -> ((isize, isize), Direction) {
    for i in 0..lines.len() as isize {
        for j in 0..lines.len() as isize {
            match Direction::try_from(lines[i as usize][j as usize]) {
                Ok(o) => return ((i, j), o),
                _ => {}
            }
        }
    }

    panic!()
}

fn visiting(lines: Vec<Vec<char>>, mut guard: (isize, isize), mut direction: Direction) -> HashSet<(isize, isize)> {
    let n = lines.len() as isize;
    let mut forward: (isize, isize) = (&direction).into();
    let mut next = (guard.0 + forward.0, guard.1 + forward.1);

    let mut visited = HashSet::new();
    while 0 <= next.0 && next.0 < n && 0 <= next.1 && next.1 < n {
        if let '#' = lines[next.0 as usize][next.1 as usize] { forward = direction.next(); }
        else {
            visited.insert(next);
            guard = next;
        }

        next = (guard.0 + forward.0, guard.1 + forward.1);
    }

    visited
}

#[allow(dead_code)]
fn print_map(map: &Vec<Vec<char>>, position: &(usize, usize), dir: &Direction) {
    let c: char = dir.into();
    let mut map_clone = map.clone();
    map_clone[position.0][position.1] = c;

    std::process::Command::new("clear").status().unwrap();
    for cell in map_clone { println!("{}", cell.iter().collect::<String>()) }
    println!("\n");
}

#[allow(dead_code)]
pub fn part_1() -> Result<usize, Box<dyn Error>> {
    let lines = file_to_chars("src/inputs/input_06.txt")?;

    let (start_guard, start_direction) = find_guard(lines.clone());

    Ok(visiting(lines, start_guard, start_direction).len())
}

#[allow(dead_code)]
pub fn part_2() -> Result<usize, Box<dyn Error>> {
    let lines = file_to_chars("src/inputs/input_06.txt")?;
    /*
    let lines = vec![
        vec!['.', '.', '.', '.', '#', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        vec!['.', '.', '#', '.', '.', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.', '#', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        vec!['.', '#', '.', '.', '^', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.', '#', '.'],
        vec!['#', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.', '#', '.', '.', '.']
    ];
    */

    let (start_guard, start_direction) = find_guard(lines.clone());
    let visited = visiting(lines.clone(), start_guard, start_direction.clone());
    let n = lines.len() as isize;

    let mut res = 0;
    for v in visited {
        let mut tmp_lines = lines.clone();
        tmp_lines[v.0 as usize][v.1 as usize] = '#';
        tmp_lines[start_guard.0 as usize][start_guard.1 as usize] = '.';

        let mut guard = start_guard;
        let mut direction = start_direction.clone();
        let mut forward: (isize, isize) = (&direction).into();
        let mut next = (guard.0 + forward.0, guard.1 + forward.1);

        let mut obstacles = HashSet::new();
        while 0 <= next.0 && next.0 < n && 0 <= next.1 && next.1 < n {
            // print_map(&tmp_lines, &(guard.0 as usize, guard.1 as usize), &direction);

            if let '#' = tmp_lines[next.0 as usize][next.1 as usize] {
                if obstacles.contains(&(next, direction.clone())) {
                    res += 1;
                    break;
                } else { obstacles.insert((next, direction.clone())); }

                forward = direction.next();
            } else { guard = next; }

            next = (guard.0 + forward.0, guard.1 + forward.1);
        }
    }

    Ok(res)
}
