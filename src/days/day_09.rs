use std::error::Error;
use crate::utils::read_file;

#[allow(dead_code)]
fn print_blocks(blocks: Vec<Option<i32>>) {
    println!("{:?}", blocks.into_iter().map(|x| match x {
        Some(s) => s.to_string(),
        None => String::from("."),
    }).collect::<Vec<String>>().join(""))
}

#[allow(dead_code)]
enum Batch {
    File(u8, u8),
    Free(u8),
}

#[allow(dead_code)]
pub fn part_1() -> Result<usize, Box<dyn Error>> {
    let data = read_file("src/inputs/input_09.txt")?;
    // let data = String::from("2333133121414131402");

    let mut is_file = true;
    let mut file_num = 0;
    let mut nb_files = 0;
    let mut blocks = Vec::new();
    for c in data.chars() {
        let i = c.to_digit(10).ok_or("Parsing impossible")?;
        match is_file {
            true  => {
                for _ in 0..i {
                    blocks.push(Some(file_num));
                    nb_files += 1;
                }
                file_num += 1;
            },
            false => for _ in 0..i { blocks.push(None) },
        }
        is_file = !is_file;
    }

    //print_blocks(blocks.clone());
    for ri in (nb_files..blocks.len()).rev() {
        if let Some(s) = blocks[ri] { for i in 0..blocks.len() {
            if let Some(_) = blocks[i] { continue; }
            blocks[i] = Some(s);
            blocks[ri] = None;
            break;
        }}
        //print_blocks(blocks.clone());
    }

    let mut res = 0;
    for i in 0..(nb_files) {
        res += i * blocks[i].ok_or("Il ne s'agit pas d'un chiffre")? as usize;
    }

    Ok(res)
}

#[allow(dead_code)]
pub fn part_2() -> Result<usize, Box<dyn Error>> {
    // let data = read_file("src/inputs/input_09.txt")?;
    // let data = String::from("2333133121414131402");

    /*
    let mut is_file = true;
    let mut file_num = 0;
    let mut nb_files = 0;
    let mut blocks = Vec::new();
    for c in data.chars() {
        let i = c.to_digit(10).ok_or("Parsing impossible")?;
        match is_file {
            true  => blocks.push(Batch::File(file_num, i as u8)),
            false => blocks.push(Batch::Free(file_num)),
        }
        is_file = !is_file;
    }

    //print_blocks(blocks.clone());
    for ri in (nb_files..blocks.len()).rev() {
        if let Some(s) = blocks[ri] { for i in 0..blocks.len() {
            if let Some(_) = blocks[i] { continue; }
            blocks[i] = Some(s);
            blocks[ri] = None;
            break;
        }}
        //print_blocks(blocks.clone());
    }

    let mut res = 0;
    for i in 0..(nb_files) {
        res += i * blocks[i].ok_or("Il ne s'agit pas d'un chiffre")? as usize;
    }

    Ok(res)
    */

    Ok(0)
}