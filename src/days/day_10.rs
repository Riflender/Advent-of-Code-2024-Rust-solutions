use std::any::TypeId;
use std::collections::HashSet;
use std::error::Error;
use std::hash::Hash;
use crate::utils::file_to_digits;

//const TYPE_ID_OF_VEC: TypeId = TypeId::of::<Vec<(usize, usize)>>();
//const TYPE_ID_OF_HASH_SET: TypeId = TypeId::of::<HashSet<(usize, usize)>>();

trait CollectionTrait {
    fn new_collection() -> Collection;
}

enum Collection {
    Vec(Vec<(usize, usize)>),
    HashSet(HashSet<(usize, usize)>),
}

impl Collection {
    fn len(&self) -> usize {
        match self {
            Collection::Vec(v) => v.len(),
            Collection::HashSet(hs) => hs.len(),
        }
    }
}

impl Extend<(usize, usize)> for Collection {
    fn extend<T: IntoIterator<Item=(usize, usize)>>(&mut self, iter: T) {
        match self {
            Collection::Vec(v) => v.extend(iter),
            Collection::HashSet(hs) => hs.extend(iter),
        }
    }
}

impl CollectionTrait for Vec<(usize, usize)> {
    fn new_collection() -> Collection {
        Collection::Vec(Vec::new())
    }
}
impl CollectionTrait for HashSet<(usize, usize)> {
    fn new_collection() -> Collection {
        Collection::HashSet(HashSet::new())
    }
}

impl<T> From<(usize, usize)> for Collection {
    fn from(value: (usize, usize)) -> Self {
        todo!()
    }
}

trait Test {}
impl Test for Vec<(usize, usize)> {}
impl Test for HashSet<(usize, usize)> {}

fn test<C, const N: usize>() -> C
where C: Test + From<[(usize, usize); N]>, {

}

fn resolution() -> Result<usize, Box<dyn Error>> {
    // let data = file_to_digits("src/inputs/input_10.txt")?;

    let data = vec![
        vec![8, 9, 0, 1, 0, 1, 2, 3],
        vec![7, 8, 1, 2, 1, 8, 7, 4],
        vec![8, 7, 4, 3, 0, 9, 6, 5],
        vec![9, 6, 5, 4, 9, 8, 7, 4],
        vec![4, 5, 6, 7, 8, 9, 0, 3],
        vec![3, 2, 0, 1, 9, 0, 1, 2],
        vec![0, 1, 3, 2, 9, 8, 0, 1],
        vec![1, 0, 4, 5, 6, 7, 3, 2]
    ];

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
        score += hiking_trail(th, 0, &data).len();
    }

    Ok(score)
}

fn hiking_trail<T: Test>(po: (usize, usize), height: u8, data: &Vec<Vec<u8>>) -> HashSet<(usize, usize)> {
    if height >= 9 { return HashSet::from([po]) }

    let mut scores = HashSet::new();
    let n = data.len() - 1;

    if po.0 > 0 && data[po.0 - 1][po.1] == height + 1 {
        scores.extend(hiking_trail((po.0 - 1, po.1), height + 1, &data));
    }
    if po.1 < n && data[po.0][po.1 + 1] == height + 1 {
        scores.extend(hiking_trail((po.0, po.1 + 1), height + 1, &data));
    }
    if po.0 < n && data[po.0 + 1][po.1] == height + 1 {
        scores.extend(hiking_trail((po.0 + 1, po.1), height + 1, &data));
    }
    if po.1 > 0 && data[po.0][po.1 - 1] == height + 1 {
        scores.extend(hiking_trail((po.0, po.1 - 1), height + 1, &data));
    }

    scores
}

#[allow(dead_code)]
pub fn part_1() -> Result<usize, Box<dyn Error>> {
    resolution()
}

#[allow(dead_code)]
pub fn part_2() -> Result<usize, Box<dyn Error>> {
    resolution()
}