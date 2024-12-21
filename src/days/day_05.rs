use std::error::Error;
use crate::utils::iterate_on_lines_2;

fn parse_line_1(line: &str) -> Result<(u8, u8), Box<dyn Error>> {
    let mut parts = line.split("|");
    let first = parts.next().ok_or("Pas assez de parties dans la chaîne")?.parse()?;
    let second = parts.next().ok_or("Pas assez de parties dans la chaîne")?.parse()?;

    Ok((first, second))
}

fn parse_line_2(line: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    Ok(line.split(",").map(|x| x.parse::<u8>().unwrap()).collect::<Vec<u8>>())
}

fn apply_rules(rules: &Vec<(u8, u8)>, update: &Vec<u8>) -> Option<u8> {
    for (a, b) in rules.iter() {
        if let (
            Some(s),
            Some(t)
        ) = (
            update.iter().position(|x| x == a),
            update.iter().position(|x| x == b)
        ) {
            if !(s < t) { return None }
        }
    }

    Some(update[update.len() / 2])
}

fn sort(rules: &Vec<(u8, u8)>, update: &mut Vec<u8>) -> Vec<u8> {
    let mut is_sorted;
    loop {
        is_sorted = true;
        for i in 0..(update.len() - 1) {
            if let Some(_) = rules.iter().position(|(a, b)| *a == update[i+1] && *b == update[i]) {
                is_sorted = false;
                (update[i], update[i + 1]) = (update[i + 1], update[i])
            }
        }
        if is_sorted { return update.to_owned() }
    }
}

#[allow(dead_code)]
pub fn part_1() -> Result<usize, Box<dyn Error>> {
    let (rules, updates) = iterate_on_lines_2(
        "src/inputs/input_05.txt",
        &mut parse_line_1,
        &mut parse_line_2
    )?;
    let mut middle = Vec::new();

    for update in updates {
        if let Some(s) = apply_rules(&rules, &update) { middle.push(s as usize); }
    }

    Ok(middle.into_iter().sum())
}

#[allow(dead_code)]
pub fn part_2() -> Result<usize, Box<dyn Error>> {
    let (rules, updates) = iterate_on_lines_2(
        "src/inputs/input_05.txt",
        &mut parse_line_1,
        &mut parse_line_2
    )?;
    let mut middle = Vec::new();

    for mut update in updates {
        if let Some(_) = apply_rules(&rules, &update) { continue; }
        
        let ordered = sort(&rules, &mut update);
        
        middle.push(ordered[ordered.len() / 2] as usize)
    }

    Ok(middle.into_iter().sum())
}
