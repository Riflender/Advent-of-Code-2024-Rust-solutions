use std::char::ParseCharError;
use std::error::Error;
use std::fs::{read_to_string, File};
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_file<P: AsRef<Path>>(filename: P) -> io::Result<String> {
    read_to_string(filename)
}

pub fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn iterate_on_lines<P, F, R>(filename: P, f: F) -> Result<Vec<R>, Box<dyn Error>>
where
    P: AsRef<Path>,
    F: Fn(&str) -> Result<R, Box<dyn Error>>,
{
    let lines = read_lines(filename)?;
    let mut vec = Vec::new();

    for line in lines.flatten() {
        if line == "" { continue; }
        vec.push(f(&line)?);
    }

    Ok(vec)
}

pub fn iterate_on_lines_2<P, F, G, R, S>(
    filename: P,
    f1: F,
    f2: G
) -> Result<(Vec<R>, Vec<S>), Box<dyn Error>>
where
    P: AsRef<Path>,
    F: Fn(&str) -> Result<R, Box<dyn Error>>,
    G: Fn(&str) -> Result<S, Box<dyn Error>>,
{
    let lines = read_lines(filename)?;
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();
    let mut section_one = true;

    for line in lines.flatten() {
        if line == "" {
            section_one = false;
            continue;
        }
        match section_one {
            true => vec1.push(f1(&line)?),
            false => vec2.push(f2(&line)?),
        }
    }

    Ok((vec1, vec2))
}

pub fn iterate_on_lines_batch<P: AsRef<Path>, R>(
    filename: P,
    batch_size: usize,
    f: &mut dyn Fn(&[String]) -> Result<R, Box<dyn Error>>
) -> Result<Vec<R>, Box<dyn Error>> {
    let lines = read_lines(filename)?;
    let mut vec = Vec::new();

    for batch in lines.flatten().collect::<Vec<String>>().chunks(batch_size) {
        vec.push(f(batch)?);
    }

    Ok(vec)
}

pub fn file_to_bytes<P: AsRef<Path>>(filename: P) -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
    Ok(read_lines(filename)?.flatten().map(|x| x.into_bytes()).collect())
}

pub fn file_to_chars<P: AsRef<Path>>(filename: P) -> Result<Vec<Vec<char>>, Box<dyn Error>> {
    Ok(read_lines(filename)?.flatten().map(|x| x.chars().collect()).collect())
}

pub fn file_to_digits<P: AsRef<Path>>(filename: P) -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
    Ok(read_lines(filename)?.flatten().map(|x| x.into_bytes().into_iter().map(|y| y - 48).collect()).collect())
}

pub fn file_to_iter_chars_than_lines<P: AsRef<Path>, R>(
    filename: P,
    f: &mut dyn Fn(&str) -> Result<Vec<R>, Box<dyn Error>>
) -> Result<(Vec<Vec<char>>, Vec<R>), Box<dyn Error>> {
    let lines = read_lines(filename)?;
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();
    let mut section_one = true;

    for line in lines.flatten() {
        if line == "" {
            section_one = false;
            continue;
        }
        match section_one {
            true => vec1.push(line.chars().collect()),
            false => vec2.extend(f(&line)?),
        }
    }

    Ok((vec1, vec2))
}

#[derive(Clone)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left
}

impl Direction {
    pub fn next(&mut self) -> (isize, isize) {
        match self {
            Direction::Up =>    { *self = Direction::Right; (0, 1) }
            Direction::Right => { *self = Direction::Down; (1, 0) }
            Direction::Down =>  { *self = Direction::Left; (0, -1) }
            Direction::Left =>   { *self = Direction::Up; (-1, 0) }
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = ParseCharError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Direction::Up),
            '>' => Ok(Direction::Right),
            'v' => Ok(Direction::Down),
            '<' => Ok(Direction::Left),
            _ => panic!("Direction wrongly parsed from char")
        }
    }
}

impl Into<char> for Direction {
    fn into(self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Right => '>',
            Direction::Down => 'v',
            Direction::Left => '<',
        }
    }
}

impl Into<(isize, isize)> for &Direction {
    fn into(self) -> (isize, isize) {
        match self {
            Direction::Up =>    (-1,  0),
            Direction::Right => ( 0,  1),
            Direction::Down =>  ( 1,  0),
            Direction::Left =>  ( 0, -1),
        }
    }
}
