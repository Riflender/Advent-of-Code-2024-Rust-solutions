use std::collections::HashSet;
use std::error::Error;
use crate::utils::file_to_digits;

trait CollectionTrait: From<[(usize, usize); 1]> + Extend<(usize, usize)> + IntoIterator<Item = (usize, usize)> {
    fn new() -> Self;
    fn len(&self) -> usize;
}
impl CollectionTrait for Vec<(usize, usize)> {
    fn new() -> Self { Self::new() }
    fn len(&self) -> usize { self.len() }
}
impl CollectionTrait for HashSet<(usize, usize)> {
    fn new() -> Self { Self::new() }
    fn len(&self) -> usize { self.len() }
}

fn resolution<C>() -> Result<usize, Box<dyn Error>>
where C: CollectionTrait, {
    let data = file_to_digits("src/inputs/input_10.txt")?;

    let n = data.len();

    let mut trailheads = Vec::new();
    for i in 0..n {
        for j in 0..n {
            if data[i][j] == 0 {
                trailheads.push((i, j));
            }
        }
    }

    let mut score = 0;
    for th in trailheads {
        score += hiking_trail::<C>(th, 0, &data).len();
    }

    Ok(score)
}

fn hiking_trail<C>(po: (usize, usize), height: u8, data: &Vec<Vec<u8>>) -> C
where C: CollectionTrait, {
    if height >= 9 { return C::from([po]) }

    let mut scores = C::new();
    let n = data.len() - 1;

    if po.0 > 0 && data[po.0 - 1][po.1] == height + 1 {
        scores.extend(hiking_trail::<C>((po.0 - 1, po.1), height + 1, &data));
    }
    if po.1 < n && data[po.0][po.1 + 1] == height + 1 {
        scores.extend(hiking_trail::<C>((po.0, po.1 + 1), height + 1, &data));
    }
    if po.0 < n && data[po.0 + 1][po.1] == height + 1 {
        scores.extend(hiking_trail::<C>((po.0 + 1, po.1), height + 1, &data));
    }
    if po.1 > 0 && data[po.0][po.1 - 1] == height + 1 {
        scores.extend(hiking_trail::<C>((po.0, po.1 - 1), height + 1, &data));
    }

    scores
}

#[allow(dead_code)]
pub fn part_1() -> Result<usize, Box<dyn Error>> {
    resolution::<HashSet<(usize, usize)>>()
}

#[allow(dead_code)]
pub fn part_2() -> Result<usize, Box<dyn Error>> {
    resolution::<Vec<(usize, usize)>>()
}