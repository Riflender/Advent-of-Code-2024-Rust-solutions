use std::error::Error;
use crate::utils::iterate_on_lines;

fn parse_line(line: &str) -> Result<usize, Box<dyn Error>> {
    match line.parse::<usize>() {
        Ok(o) => Ok(o),
        Err(e) => Err(Box::from(e))
    }
}

fn next_secret(mut secret: usize) -> usize {
    secret = (secret * 64 ^ secret) % 16777216;
    secret = (secret / 32 ^ secret) % 16777216;
    (secret * 2048 ^ secret) % 16777216
}

#[allow(dead_code)]
pub fn part_1() -> Result<usize, Box<dyn Error>> {
    let lines = iterate_on_lines("src/inputs/input_22.txt", parse_line)?;

    let mut res = 0;
    for mut line in lines {
        for _ in 0..2000 { line = next_secret(line); }
        res += line;
    }

    Ok(res)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Sequence((i8, i8, i8, i8));

impl Sequence {
    fn new() -> Self {
        Sequence((-9, -9, -9, -9))
    }

    fn next(&mut self) {
        match self.0 {
            (9, 9, 9, 9) => self.0 = (-9, -9, -9, -9),
            (9, 9, 9, _) => {
                self.0.0 = -9;
                self.0.1 = -9;
                self.0.2 = -9;
                self.0.3 += 1;
            }
            (9, 9, _, _) => {
                self.0.0 = -9;
                self.0.1 = -9;
                self.0.2 += 1;
            }
            (9, _, _, _) => {
                self.0.0 = -9;
                self.0.1 += 1;
            }
            (_, _, _, _) => self.0.0 += 1,
        }
    }
}

#[allow(dead_code)]
pub fn part_2() -> Result<u128, Box<dyn Error>> {
    // let lines = iterate_on_lines("src/inputs/input_22.txt", parse_line)?;
    let lines = vec![1, 2, 3, 2024];

    let mut all_secrets = Vec::new();
    for mut line in lines {
        let mut secrets = Vec::new();
        secrets.push(((line % 10) as i8, -10_i8));

        for _ in 0..2000 {
            line = next_secret(line);
            let price = (line % 10) as i8;
            let (x, _) = secrets.last().expect("Vec should not be empty");
            secrets.push((price, price - x.clone()));
        }

        all_secrets.push(secrets);
    }

    let mut sequence = Sequence::new();
    let mut res = (Sequence::new(), 0);

    for i in 0..19_usize.pow(4) {
        // if i % 19_usize.pow(3) == 0 { println!("{:?}", sequence); }
        let mut tmp_res = 0;

        for secrets in all_secrets.clone() {
            //let mut queue = VecDeque::with_capacity(4);
            for secret in secrets.windows(4) {
                if sequence.0 == (secret[0].1, secret[1].1, secret[2].1, secret[3].1) {
                    tmp_res += secret[3].0 as u128;
                    break;
                }
            }
        }

        if tmp_res > res.1 {
            res = (sequence, tmp_res);
        }
        sequence.next();
    }

    Ok(res.1)
}