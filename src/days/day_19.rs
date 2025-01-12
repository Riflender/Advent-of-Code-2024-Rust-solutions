use std::error::Error;
use crate::utils::iterate_on_lines_2;

fn parse_line_1(line: &str) -> Result<Vec<String>, Box<dyn Error>> {
    Ok(line.split(", ").map(|x| x.to_string()).collect::<Vec<String>>())
}

fn parse_line_2(line: &str) -> Result<String, Box<dyn Error>> { Ok(line.to_string()) }

fn rec_made(word: &str, towels: &Vec<String>, mult: bool) -> Result<usize, Box<dyn Error>> {
    if word.len() == 0 {
        return Ok(1)
    }
    let mut possibles = 0;

    for t in towels {
        if t.len() > word.len() { break; }
        let towel = t.as_str();

        if word.starts_with(towel) {
            if let Ok(p) = rec_made(&word[towel.len()..], towels, mult) {
                if p == 0 { continue; }
                //println!("{word}");
                if mult { possibles += p; }
                else { return Ok(1) }
            }
        }
    }

    Ok(possibles)
}

fn run(mult: bool)  -> Result<usize, Box<dyn Error>> {
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
    for word in words {
        let tmp = rec_made(&word, &towels, mult)?;
        //println!("{word} : {tmp}");
        possibles += tmp;
    }

    Ok(possibles)
}

#[allow(dead_code)]
pub fn part_1() -> Result<usize, Box<dyn Error>> {
    run(false)
}

#[allow(dead_code)]
pub fn part_2() -> Result<usize, Box<dyn Error>> {
    run(true)
}