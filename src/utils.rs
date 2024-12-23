use std::error::Error;
use std::fs::{File, read_to_string};
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_file<P>(filename: P) -> io::Result<String>
where P: AsRef<Path>, {
    read_to_string(filename)
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn iterate_on_lines<P, R>(filename: P, f: &mut dyn Fn(&str) -> Result<R, Box<dyn Error>>) -> Result<Vec<R>, Box<dyn Error>>
where P: AsRef<Path>, {
    let lines = read_lines(filename)?;
    let mut vec = Vec::new();

    for line in lines.flatten() {
        if line == "" { continue; }
        vec.push(f(&line)?);
    }

    Ok(vec)
}

pub fn iterate_on_lines_2<P, R, S>(
    filename: P,
    f1: &mut dyn Fn(&str) -> Result<R, Box<dyn Error>>,
    f2: &mut dyn Fn(&str) -> Result<S, Box<dyn Error>>
) -> Result<(Vec<R>, Vec<S>), Box<dyn Error>>
where P: AsRef<Path>, {
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

pub fn iterate_on_lines_batch<P, R>(filename: P, batch_size: usize, f: &mut dyn Fn(&[String]) -> Result<R, Box<dyn Error>>) -> Result<Vec<R>, Box<dyn Error>>
where P: AsRef<Path>, {
    let lines = read_lines(filename)?;
    let mut vec = Vec::new();

    for batch in lines.flatten().collect::<Vec<String>>().chunks(batch_size) {
        vec.push(f(batch)?);
    }

    Ok(vec)
}

pub fn file_to_bytes<P>(filename: P) -> Result<Vec<Vec<u8>>, Box<dyn Error>>
where P: AsRef<Path>, {
    Ok(read_lines(filename)?.flatten().map(|x| x.into_bytes()).collect())
}

pub fn file_to_chars<P>(filename: P) -> Result<Vec<Vec<char>>, Box<dyn Error>>
where P: AsRef<Path>, {
    Ok(read_lines(filename)?.flatten().map(|x| x.chars().collect()).collect())
}

pub fn file_to_digits<P>(filename: P) -> Result<Vec<Vec<u8>>, Box<dyn Error>>
where P: AsRef<Path>, {
    Ok(read_lines(filename)?.flatten().map(|x| x.into_bytes().into_iter().map(|y| y - 48).collect()).collect())
}