use std::time::SystemTime;
use std::thread;
use std::error::Error;
use std::thread::JoinHandle;
use crate::utils::read_file;

fn resolution(nb_blinks: usize) -> Result<usize, Box<dyn Error>> {
    let stones = read_file("src/inputs/input_11.txt")?
        .split(" ")
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let threads: Vec<JoinHandle<Result<usize, Box<dyn Error + Send + Sync>>>> = stones.into_iter().map(|x| {
        thread::spawn(move || { blinks(vec![x], nb_blinks) })
    }).collect();

    Ok(threads
        .into_iter()
        .map(|handle| handle.join().unwrap().unwrap())
        .collect::<Vec<usize>>()
        .iter()
        .sum())

    //blinks(stones, nb_blinks)
}

fn blinks(mut stones: Vec<u64>, nb_blinks: usize) -> Result<usize, Box<dyn Error + Send + Sync>> {
    let mut previous_time = SystemTime::now();
    for i in 0..nb_blinks {
        println!("{i}: {},{}", previous_time.elapsed()?.as_secs(), previous_time.elapsed()?.subsec_nanos());
        previous_time = SystemTime::now();

        let mut j = 0;

        while j < stones.len() {
            if stones[j] == 0 { stones[j] = 1; }
            else if stones[j].to_string().len() % 2 == 0 {
                let s = stones[j].to_string();
                stones.insert(j + 1, s.as_str()[s.len() / 2..].parse()?);
                stones[j] = s.as_str()[..s.len() / 2].parse()?;
                j += 1;
            }
            else { stones[j] = stones[j] * 2024; }

            j += 1;
        }
    }

    Ok(stones.len())
}

#[allow(dead_code)]
pub fn part_1() -> Result<usize, Box<dyn Error>> {
    resolution(25)
}

#[allow(dead_code)]
pub fn part_2() -> Result<usize, Box<dyn Error>> {
    resolution(75)
}