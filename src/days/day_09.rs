use std::collections::VecDeque;
use std::error::Error;
use crate::utils::read_file;

#[allow(dead_code)]
fn print_blocks(blocks: Vec<i32>) {
    println!("{:?}", blocks.into_iter().map(|x| match x {
        i if i >= 0 => i.to_string(),
        _ => String::from("."),
    }).collect::<Vec<String>>().join(""))
}

#[allow(dead_code)]
pub fn part_1() -> Result<usize, Box<dyn Error>> {
    let data = read_file("src/inputs/input_09.txt")?;
    //let data = String::from("2333133121414131402");

    let mut is_file = true;
    let mut file_num = 0;
    let mut nb_files = 0;
    let mut blocks = Vec::new();
    let mut frees = VecDeque::new();

    for c in data.chars() {
        let i = c.to_digit(10).ok_or("Parsing impossible")?;

        match is_file {
            true  => {
                for _ in 0..i {
                    blocks.push(file_num);
                    nb_files += 1;
                }
                file_num += 1;
            },
            false => for _ in 0..i {
                frees.push_back(blocks.len());
                blocks.push(-1);
            },
        }

        is_file = !is_file;
    }

    // print_blocks(blocks.clone());
    for ri in (nb_files..blocks.len()).rev() {
        if blocks[ri] >= 0 {
            if let Some(f) = frees.front() {
                blocks[*f] = blocks[ri];
                blocks[ri] = -1;
                frees.pop_front();
            } else { break; }
        }
        // print_blocks(blocks.clone());
    }

    let mut res = 0;
    for i in 0..nb_files {
        res += i * blocks[i] as usize;
    }

    Ok(res)
}

#[derive(Debug, Clone)]
struct DataSpace {
    index: usize,
    len: usize,
    value: i32,
}

impl DataSpace {
    fn new(index: usize, len: usize, value: i32) -> Self { DataSpace { index, len, value } }
}

#[allow(dead_code)]
pub fn part_2() -> Result<usize, Box<dyn Error>> {
    let data = read_file("src/inputs/input_09.txt")?;
    //let data = String::from("2333133121414131402");

    let mut is_file = true;
    let mut file_num = 0;
    let mut blocks = Vec::new();
    let mut files = VecDeque::new();
    let mut frees = VecDeque::new();

    for c in data.chars() {
        let i = c.to_digit(10).ok_or("Parsing impossible")? as usize;
        if i == 0 {
            is_file = !is_file;
            continue;
        }

        match is_file {
            true  => {
                files.push_front(DataSpace::new(blocks.len(), i, file_num));
                for _ in 0..i { blocks.push(file_num); }
                file_num += 1;
            }
            false => {
                frees.push_back(DataSpace::new(blocks.len(), i, -1));
                for _ in 0..i { blocks.push(-1); }
            }
        }

        is_file = !is_file;
    }

    // print_blocks(blocks.clone());
    for file in files {
        for (i, free) in frees.iter_mut().enumerate() {
            if free.index > file.index { break; }
            else if free.len < file.len { continue; }

            for j in free.index..free.index+file.len {
                blocks[j] = file.value;
            }
            for j in file.index..file.index+file.len {
                blocks[j] = -1;
            }

            if free.len == file.len {
                frees.remove(i);
            } else if free.len > file.len {
                free.index += file.len;
                free.len -= file.len;
            }

            // print_blocks(blocks.clone());
            break;
        }
    }

    let mut res = 0;
    for i in 0..blocks.len() {
        if blocks[i] < 0 { continue; }
        res += i * blocks[i] as usize;
    }

    Ok(res)
}