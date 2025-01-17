use std::collections::HashSet;
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

fn make_sequences(sequence: &String, mut robot: (i8, i8), f: fn(char) -> (i8, i8)) -> Vec<String> {
    let mut res = Vec::new();

    for c in sequence.chars() {
        let next = f(c);
        let delta = (next.0 - robot.0, next.1 - robot.1);

        let moves = format!(
            "{}{}",
            if delta.0.is_positive() { "v" } else { "^" }.repeat(delta.0.abs() as usize),
            if delta.1.is_positive() { ">" } else { "<" }.repeat(delta.1.abs() as usize),
        );

        let sub_sequences = moves
            .chars()
            .permutations(moves.len())
            .collect::<HashSet<Vec<char>>>()
            .into_iter()
            .map(|mut x| {
                x.push('A');
                x.into_iter().collect::<String>()
            })
            .collect::<Vec<String>>();

        robot = next;

        res.push(sub_sequences);
    }

    res.into_iter()
        .multi_cartesian_product()
        .map(|x| x.join(""))
        .collect::<Vec<String>>()
}

#[allow(dead_code)]
pub fn part_1() -> Result<usize, Box<dyn Error>> {
    let lines = iterate_on_lines("src/inputs/input_21.txt", |x| Ok(x.to_string()))?;
    //let lines = vec![String::from("029A"), String::from("980A"), String::from("179A"), String::from("456A"), String::from("379A"), ];

    let mut sequences = Vec::new();
    let mut i = 4;

    //for line in lines.clone() {
    let line = lines[4].clone();
        let r1 = make_sequences(&line, (3, 2), translate_numeric_keypad);

        let mut r2 =  Vec::new();
        for r in r1 {
            r2.append(&mut make_sequences(&r, (0, 2), translate_directional_keypad));
        }
        r2.sort_by(|x, y| x.len().cmp(&y.len()));

        let length = r2.first().ok_or("Empty vector")?.len();
        r2.retain(|x| x.len() == length);
        i += 1;
        println!("Sequence n°{i}/5 : len == {}", r2.len());

        let mut r3 =  Vec::new();
        let mut j = 0;
        let k = r2.len();
        for r in r2 {
            j += 1;
            println!("{j}/{k}");
            r3.append(&mut make_sequences(&r, (0, 2), translate_directional_keypad));
        }

        sequences.push(r3.into_iter().min_by(|x, y| x.len().cmp(&y.len())).ok_or("Empty vector")?);

        let mut tmp = line.clone();
        tmp.pop();
        let num = tmp.as_str().parse::<usize>()?;
        let sequence = sequences.last().unwrap();
        println!("{line}: {} * {num} = {}\t{:?}", sequence.len(), sequence.len() * num, sequence);
    //}

    let mut res = 0;
    for (line, sequence) in zip(lines, sequences) {
        let mut tmp = line.clone();
        tmp.pop();
        let num = tmp.as_str().parse::<usize>()?;

        println!("{line}: {} * {num} = {}\t{:?}", sequence.len(), sequence.len() * num, sequence);
        res += sequence.len() * num;
    }

    Ok(res)

    // 480A: 70 * 480 = 33600  "v<<AA>A^>AA<Av>A^AAvA^Av<<A^>>Av<A>A^A<A>A<v<A>A>^AAAvA^<A>Av<A^>A<A>A"
    // 682A: 68 * 682 = 46376  "<<vA>^>AAvA^Av<A<AA>^>AvA^<A>AvA^Av<A<A>>^AAvA^<A>A<vA<A>>^AvA^A<A>A"
    // 140A: 62 * 140 = 8680   "v<A<AA>^>AA<Av>A^AvA^Av<<A>>^AvA^Av<A<A>^>AAvA^A<A>A<vA^>A<A>A"
    // 246A: 70 * 246 = 17220  "v<<AA>A>^A<Av>A^AvA^Av<A<AA^>>A<Av>A^AvA^Av<A^>AA<A>Av<<A>A^>AA<A>vA^A"
    // 938A: 72 * 938 = 67536  "v<<A^>>AAAvA^A<vA<A>>^AAvA^<A>A<vA<AA>^>AvA^<A>AAvA^Av<A<A>^>AAAvA^A<A>A"
}

#[allow(dead_code)]
pub fn part_2() -> Result<usize, Box<dyn Error>> {
    let _lines = read_file("src/inputs/input_21.txt")?;



    Ok(0)
}