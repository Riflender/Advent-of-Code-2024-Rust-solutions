use std::collections::VecDeque;
use std::error::Error;
use crate::utils::iterate_on_lines;

fn parse_line(line: &str) -> Result<usize, Box<dyn Error>> {
    match line.parse::<usize>() {
        Ok(o) => Ok(o),
        Err(e) => Err(Box::from(e))
    }
}

#[allow(dead_code)]
pub fn part_1() -> Result<usize, Box<dyn Error>> {
    let lines = iterate_on_lines("src/inputs/input_22.txt", parse_line)?;

    let mut res = 0;
    for mut line in lines {
        for _ in 0..2000 {
            line = (line * 64 ^ line) % 16777216;
            line = (line / 32 ^ line) % 16777216;
            line = (line * 2048 ^ line) % 16777216;
        }
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

    /*
    fn compare_to(&self, vec: VecDeque<i8>) -> bool {
        let self_vec = VecDeque::from(vec![self.0.3, self.0.2, self.0.1, self.0.0]);
        self_vec == vec
    }

    fn compare_to(&self, vec: VecDeque<(i8, Option<i8>)>) -> bool {
        let mut tmp = (-10, -10, -10, -10);
        let mut it = vec.iter();

        let f = |x: &mut Iter<(i8, Option<i8>)>| x.next().expect("Missing value in Dequeue").1.expect("Missing diff value in Dequeue");

        tmp.0 = f(&mut it);
        tmp.1 = f(&mut it);
        tmp.2 = f(&mut it);
        tmp.3 = f(&mut it);

        println!("{:?}\t{:?}", self, tmp);

        *self == Sequence(tmp)
    }
    */

    fn compare_to(&self, sl: &mut [(i8, Option<i8>)]) -> bool {
        match sl {
            &mut [(_, Some(a)), (_, Some(b)), (_, Some(c)), (_, Some(d))] => self.0 == (a, b, c, d),
            &mut _ => false,
        }
    }
}

#[allow(dead_code)]
pub fn part_2() -> Result<usize, Box<dyn Error>> {
    // let lines = iterate_on_lines("src/inputs/input_22.txt", parse_line)?;
    let lines = vec![1, 2, 3, 2024];

    let mut sequence = Sequence::new();
    let mut res = (Sequence::new(), 0);

    for i in 0..19_usize.pow(4) {
        if i % 19_usize.pow(3) == 0 { println!("{:?}", sequence); }
        let mut tmp_res = 0;

        for line in lines.clone() {
            let mut queue = VecDeque::with_capacity(4);
            // Pousse la premiere valeur
            queue.push_back(((line % 10) as i8, None));
            //println!("{}\t: {}", line, (line % 10) as i8);

            let mut tmp = line;
            for _ in 0..2000 {
                tmp = (tmp * 64 ^ tmp) % 16777216;
                tmp = (tmp / 32 ^ tmp) % 16777216;
                tmp = (tmp * 2048 ^ tmp) % 16777216;

                let price = (tmp % 10) as i8;

                // Récupere la precedente valeur
                let (x, _) = queue.back().expect("Queue should not be empty");
                let diff = price - x.clone();
                //println!("{tmp}\t: {price} ({diff})\t{:?}", queue);
                // Pousse la prochaine valeur
                if queue.capacity() == queue.len() { queue.pop_front(); }
                queue.push_back((price, Some(diff)));


                if sequence.compare_to(queue.make_contiguous()) {
                    tmp_res += tmp;
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