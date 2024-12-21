use std::collections::{HashMap, HashSet};
use std::error::Error;
use crate::utils::file_to_chars;

fn get_antennas(lines: Vec<Vec<char>>) -> HashMap<char, Vec<(isize, isize)>> {
    let mut res = HashMap::<char, Vec<(isize, isize)>>::new();

    for i in 0..lines.len() {
        for j in 0..lines[0].len() {
            match lines[i][j] {
                '.' => continue,
                c => res.entry(c).or_insert(Vec::new()).push((i as isize, j as isize)),
            }
        }
    }

    res
}

#[allow(dead_code)]
pub fn part_1() -> Result<usize, Box<dyn Error>> {
    let lines = file_to_chars("src/inputs/input_08.txt")?;
    let n = lines.len() as isize;
    let hm =  get_antennas(lines);

    let mut antinodes = HashSet::new();
    for (_, value) in hm {
        for i in 0..value.len() {
            for j in i + 1..value.len() {
                let a = (value[i].0 - (value[j].0 - value[i].0), value[i].1 - (value[j].1 - value[i].1));
                let b = (value[j].0 + (value[j].0 - value[i].0), value[j].1 + (value[j].1 - value[i].1));

                if 0 <= a.0 && a.0 < n && 0 <= a.1 && a.1 < n { antinodes.insert(a); }
                if 0 <= b.0 && b.0 < n && 0 <= b.1 && b.1 < n { antinodes.insert(b); }
            }
        }
    }

    Ok(antinodes.len())
}

#[allow(dead_code)]
pub fn part_2() -> Result<usize, Box<dyn Error>> {
    let lines = file_to_chars("src/inputs/input_08.txt")?;
    let n = lines.len() as isize;
    let hm = get_antennas(lines);

    let mut antinodes = HashSet::new();
    for (_, value) in hm {
        for i in 0..value.len() {
            for j in i + 1..value.len() {
                let mut a = (value[i].0, value[i].1);
                let mut b = (value[j].0, value[j].1);

                let mut k = 1;
                while 0 <= a.0 && a.0 < n && 0 <= a.1 && a.1 < n {
                    antinodes.insert(a);
                    a = (value[i].0 - (value[j].0 - value[i].0) * k, value[i].1 - (value[j].1 - value[i].1) * k);
                    k += 1;
                }

                k = 1;
                while 0 <= b.0 && b.0 < n && 0 <= b.1 && b.1 < n {
                    antinodes.insert(b);
                    b = (value[j].0 + (value[j].0 - value[i].0) * k, value[j].1 + (value[j].1 - value[i].1) * k);
                    k += 1;
                }
            }
        }
    }

    Ok(antinodes.len())
}
