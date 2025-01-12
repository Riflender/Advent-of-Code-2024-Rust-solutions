use std::error::Error;
use std::iter::zip;
use itertools::Itertools;
use crate::utils::{iterate_on_lines, read_file};

fn translate_numeric_keypad(c: char) -> (i8, i8) {
    match c {
        '7' => (0,0),
        '8' => (0,1),
        '9' => (0,2),
        '4' => (1,0),
        '5' => (1,1),
        '6' => (1,2),
        '1' => (2,0),
        '2' => (2,1),
        '3' => (2,2),
        '0' => (3,1),
        'A' => (3,2),
        _ => panic!()
    }
}

fn translate_directional_keypad(c: char) -> (i8, i8) {
    match c {
        '^' => (0,1),
        'A' => (0,2),
        '<' => (1,0),
        'v' => (1,1),
        '>' => (1,2),
        _ => panic!()
    }
}

fn translate_coord(coord: (i8, i8)) -> String {
    format!(
        "{}{}A",
        if coord.0.is_positive() { "v" } else { "^" }.repeat(coord.0.abs() as usize),
        if coord.1.is_positive() { ">" } else { "<" }.repeat(coord.1.abs() as usize),
    )
}



#[allow(dead_code)]
pub fn part_1() -> Result<usize, Box<dyn Error>> {
    //let lines = iterate_on_lines("src/inputs/input_21.txt", |x| Ok(x.to_string()))?;
    let lines = vec![String::from("029A"), String::from("980A"), String::from("179A"), String::from("456A"), String::from("379A"), ];

    let mut sequences = Vec::new();
    for line in lines.clone() {
        let mut r1 = (3, 2);
        let mut r2 = (0, 2);
        let mut r3 = (0, 2);
        let mut s = String::new();

        for c in line.chars() {
            let k = translate_numeric_keypad(c);
            let t = translate_coord((k.0 - r1.0, k.1 - r1.1));
            r1 = k;

            for d in t.chars() {
                let l = translate_directional_keypad(d);
                let u = translate_coord((l.0 - r2.0, l.1 - r2.1));
                r2 = l;

                for e in u.chars() {
                    let m = translate_directional_keypad(e);
                    let v = translate_coord((m.0 - r3.0, m.1 - r3.1));
                    r3 = m;

                    s.push_str(&v);
                }
            }
        }

        sequences.push(s);
    }

    let mut res = 0;
    for (line, sequence) in zip(lines, sequences) {
        let mut tmp = line.clone();
        tmp.pop();
        let num = tmp.as_str().parse::<usize>()?;

        println!("{line}: {} * {num} = {}\t{sequence}", sequence.len(), sequence.len() * num);
        res += sequence.len() * num;
    }

    Ok(res)
}

#[allow(dead_code)]
pub fn part_2() -> Result<usize, Box<dyn Error>> {
    let _lines = read_file("src/inputs/input_21.txt")?;



    Ok(0)
}