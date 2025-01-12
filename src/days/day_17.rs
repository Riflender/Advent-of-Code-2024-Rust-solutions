use std::cell::Cell;
use std::error::Error;
use crate::utils::iterate_on_lines;

fn init() -> Result<(Cell<usize>, Cell<usize>, Cell<usize>, Vec<usize>), Box<dyn Error>> {
    let lines = iterate_on_lines("src/inputs/input_17.txt", |x| Ok(x.to_string()))?;

    let a = Cell::new(lines[0][12..].parse::<usize>()?);
    let b = Cell::new(lines[1][12..].parse::<usize>()?);
    let c = Cell::new(lines[2][12..].parse::<usize>()?);
    let program = lines[3][9..]
        .split(',')
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    /*
    let a = Cell::new(729);
    let b = Cell::new(0);
    let c = Cell::new(0);
    let program = vec![0, 1, 5, 4, 3, 0];

    let a = Cell::new(2024);
    let b = Cell::new(0);
    let c = Cell::new(0);
    let program = vec![0, 3, 5, 4, 3, 0];
    */

    Ok((a, b, c, program))
}

fn run(a: Cell<usize>, b: Cell<usize>, c: Cell<usize>, program: Vec<usize>) -> Result<Vec<usize>, Box<dyn Error>> {
    let mut i: usize = 0;
    let mut res: Vec<usize> = Vec::new();
    let combo = |x| match x {
        0_usize..=3_usize => x,
        4 => a.get(),
        5 => b.get(),
        6 => c.get(),
        _ => panic!()
    };

    loop {
        if i >= program.len() { break; }

        match program[i] {
            0 => a.set(a.get() / 2_usize.pow(combo(program[i + 1]) as u32)),
            1 => b.set(b.get() ^ program[i + 1]),
            2 => b.set(combo(program[i + 1]) % 8),
            3 => if a.get() != 0 {
                i = program[i + 1];
                continue;
            },
            4 => b.set(b.get() ^ c.get()),
            5 => res.push(combo(program[i + 1]) % 8),
            6 => b.set(a.get() / 2_usize.pow(combo(program[i + 1]) as u32)),
            7 => c.set(a.get() / 2_usize.pow(combo(program[i + 1]) as u32)),
            _ => return Err(Box::from(""))
        }

        i += 2;
    }

    Ok(res)
}

#[allow(dead_code)]
pub fn part_1() -> Result<String, Box<dyn Error>> {
    let (a, b, c, program) = init()?;
    Ok(
        run(a, b, c, program)?
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",")
    )
}

fn rec_increment(
    b: Cell<usize>,
    c: Cell<usize>,
    program: Vec<usize>,
    n: usize,
    mut i: usize
) -> Result<usize, Box<dyn Error>> {
    let a = Cell::new(i);
    for _ in 0..8 {
        let tmp = run(a.clone(), b.clone(), c.clone(), program.clone())?;

        // 1. Rechercher les séries de 16 sorties : A/8 sert de condition de boucle ===> 8^(15) (len = pow + 1)
        // 2. Le dernier chiffre change tout les 8^(15) pour suivre l'ordre 4,6,7,0,1,2,3 ===> 8^(15) * 4

        if tmp[n] == program[n] && n != 0 {
            let res = rec_increment(b.clone(), c.clone(), program.clone(), n - 1, i)?;
            if res != 0 { return Ok(res); }
        }
        if tmp == program { return Ok(a.get()) }

        i += 8_usize.pow(n as u32);
        a.set(i);
    }

    Ok(0)
}

#[allow(dead_code)]
pub fn part_2() -> Result<usize, Box<dyn Error>> {
    let (_, b, c, program) = init()?;
    let n = program.len() - 1;
    rec_increment(b, c, program, n, 8_usize.pow(n as u32))
}