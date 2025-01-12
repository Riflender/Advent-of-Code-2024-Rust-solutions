use std::collections::HashMap;
use std::error::Error;
use crate::utils::iterate_on_lines_2;

fn parse_line_1(line: &str) -> Result<Vec<String>, Box<dyn Error>> {
    Ok(line.split(", ").map(|x| x.to_string()).collect::<Vec<String>>())
}

fn parse_line_2(line: &str) -> Result<String, Box<dyn Error>> { Ok(line.to_string()) }

fn rec_is_possible(word: &str, towels: &Vec<String>) -> Result<bool, Box<dyn Error>> {
    if word.len() == 0 { return Ok(true) }

    for towel in towels {
        if towel.len() > word.len() { break; }

        if word.starts_with(towel) {
            if let Ok(true) = rec_is_possible(&word[towel.len()..], towels) { return Ok(true) }
        }
    }

    Ok(false)
}

fn rec_count_possible(word: &str, towels: &Vec<String>, cache: &mut HashMap<String, usize>) -> Result<usize, Box<dyn Error>> {
    if let Some(s) = cache.get(word) { return Ok(*s) }
    if word.len() == 0 { return Ok(1) }

    let mut possibles = 0;
    for towel in towels {
        if towel.len() > word.len() { break; }

        if word.starts_with(towel) {
            possibles += rec_count_possible(&word[towel.len()..], towels, cache)?;
        }
    }

    cache.insert(word.to_string(), possibles);
    Ok(possibles)
}

#[allow(dead_code)]
pub fn part_1() -> Result<usize, Box<dyn Error>> {
    let (mut towels, words) = {
        let tmp = iterate_on_lines_2(
            "src/inputs/input_19.txt",
            parse_line_1,
            parse_line_2
        )?;
        (tmp.0.into_iter().flatten().collect::<Vec<String>>(), tmp.1)
    };
    //let mut towels = vec![String::from("r"), String::from("wr"), String::from("b"), String::from("g"), String::from("bwu"), String::from("rb"), String::from("gb"), String::from("br")];
    //let words = vec![String::from("brwrr"), String::from("bggr"), String::from("gbbr"), String::from("rrbgbr"), String::from("ubwu"), String::from("bwurrg"), String::from("brgr"), String::from("bbrgwb")];

    towels.sort_by(|x1, x2|
        if x1.len() == x2.len() { x1.cmp(x2) }
        else { x1.len().cmp(&x2.len()) }
    );

    let mut possibles = 0;
    for word in words { if rec_is_possible(&word, &towels)? { possibles += 1; } }

    Ok(possibles)
}

#[allow(dead_code)]
pub fn part_2() -> Result<usize, Box<dyn Error>> {
    let (mut towels, words) = {
        let tmp = iterate_on_lines_2(
            "src/inputs/input_19.txt",
            parse_line_1,
            parse_line_2
        )?;
        (tmp.0.into_iter().flatten().collect::<Vec<String>>(), tmp.1)
    };
    //let mut towels = vec![String::from("r"), String::from("wr"), String::from("b"), String::from("g"), String::from("bwu"), String::from("rb"), String::from("gb"), String::from("br")];
    //let words = vec![String::from("brwrr"), String::from("bggr"), String::from("gbbr"), String::from("rrbgbr"), String::from("ubwu"), String::from("bwurrg"), String::from("brgr"), String::from("bbrgwb")];

    towels.sort_by(|x1, x2|
        if x1.len() == x2.len() { x1.cmp(x2) }
        else { x1.len().cmp(&x2.len()) }
    );

    let mut possibles = 0;
    let mut cache: HashMap<String, usize> = HashMap::new();
    for word in words {
        possibles += rec_count_possible(&word, &towels, &mut cache)?;
    }

    Ok(possibles)
}