use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::sync::{Arc, RwLock};
use std::sync::mpsc::channel;
use threadpool::ThreadPool;
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

    fn new_with_last(last: i8) -> Self {
        if last < -9 || last > 9 { panic!(); }
        Sequence((-9, -9, -9, last))
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
    let lines = iterate_on_lines("src/inputs/input_22.txt", parse_line)?;
    // let lines = vec![1, 2, 3, 2024];

    let mut all_secrets = Vec::new();
    for mut line in lines {
        let mut hash_secrets = HashMap::new();
        hash_secrets.insert((-10_i8, -10_i8, -10_i8, -10_i8), (line % 10) as i8);

        let mut queue = VecDeque::with_capacity(4);
        let mut previous_line = (line % 10) as i8;

        for _ in 0..2000 {
            line = next_secret(line);
            let price = (line % 10) as i8;

            if queue.capacity() == queue.len() { queue.pop_front(); }
            queue.push_back(price - previous_line);

            if let Some(sl) = queue.make_contiguous().windows(4).next() {
                let tmp = (sl[0], sl[1], sl[2], sl[3]);
                if let None = hash_secrets.get(&tmp) {
                    hash_secrets.insert(tmp, price);
                }
            }

            previous_line = price;
        }

        all_secrets.push(hash_secrets);
    }
    let all_secrets_ptr = Arc::new(RwLock::new(all_secrets));

    let pool = ThreadPool::new(8);
    let (tx, rx) = channel();

    for d in -9..10 {
        let tx = tx.clone();
        let all_secrets_ptr_clone = all_secrets_ptr.clone();
        pool.execute(move || {
            let mut sequence = Sequence::new_with_last(d);
            let mut res = (Sequence::new(), 0);

            for i in 0..19_usize.pow(3) {
                if i % 19_usize.pow(2) == 0 { println!("{:?}", sequence); }
                let mut tmp_res = 0;

                for secrets in all_secrets_ptr_clone.read().expect("RwLock poisoned").clone() {
                    if let Some(price) = secrets.get(&sequence.0) {
                        tmp_res += *price as u128;
                    }
                }

                if tmp_res > res.1 {
                    res = (sequence, tmp_res);
                }

                sequence.next();
            }

            tx.send(res).expect("channel will be there waiting for the pool");
        });
    }

    let mut res = (Sequence::new(), 0);
    for (seq, most) in rx.iter().take(19) {
        if most > res.1 { res = (seq, most) }
    }

    // (2, 0, -1, 2) --> 1998
    println!("\n{:?}", res.0);
    Ok(res.1)
}