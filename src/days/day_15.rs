use std::error::Error;
use std::ptr::NonNull;
use crate::utils::{file_to_iter_chars_than_lines, Direction};

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
struct BinaryTree<T> {
    left: Option<NonNull<BinaryTree<T>>>,
    right: Option<NonNull<BinaryTree<T>>>,
    prev: Option<NonNull<BinaryTree<T>>>,
    element: T,
    len: usize,
}

impl<T> BinaryTree<T> {
    fn new(element: T) -> Self {
        BinaryTree { left: None, right: None, prev: None, element, len: 0 }
    }

    fn push(&mut self, mut left: BinaryTree<T>, mut right: BinaryTree<T>) {
        self.left = Some(NonNull::<BinaryTree<T>>::new(&mut left as *mut _).expect("ptr is null"));
        self.right = Some(NonNull::<BinaryTree<T>>::new(&mut right as *mut _).expect("ptr is null"));
    }

    fn push_one(&mut self, mut elt: BinaryTree<T>) {
        self.left = Some(NonNull::<BinaryTree<T>>::new(&mut elt as *mut _).expect("ptr is null"));
    }

    fn is_wall_inside(&self) -> bool {
        if let Some(s) = self.left {
            s.as_ref()
        }
    }
}

fn parse_line(line: &str) -> Result<Vec<Direction>, Box<dyn Error>> {
    Ok(line.chars().map(|x| Direction::try_from(x).unwrap()).collect())
}

fn forward(dir: &Direction, coord: &(usize, usize)) -> (usize, usize) {
    let t: (isize, isize) = dir.into();
    ((coord.0 as isize + t.0) as usize, (coord.1 as isize + t.1) as usize)
}

fn recursive_move(dir: &Direction, map: &mut Vec<Vec<char>>, coord: (usize, usize)) -> (usize, usize) {
    let forward = forward(dir, &coord);

    let f = |m: &mut Vec<Vec<char>>| {
        m[forward.0][forward.1] = m[coord.0][coord.1];
        m[coord.0][coord.1] = '.';
        forward
    };

    match map[forward.0][forward.1] {
        '.' => f(map),
        '#' => coord,
        _ => { if recursive_move(dir, map, forward) != forward { f(map) } else { coord } }
    }
}

fn build_tree(dir: &Direction, map: &mut Vec<Vec<char>>, coord: (usize, usize)) -> BinaryTree<(char, (usize, usize))> {
    let mut node = BinaryTree::new((map[coord.0][coord.1], coord));
    if map[coord.0][coord.1] == '.' || map[coord.0][coord.1] == '#' { return node }

    let forward = forward(dir, &coord);
    match map[forward.0][forward.1] {
        '[' if node.element.0 == '[' => node.push_one(build_tree(dir, map, forward)),
        ']' if node.element.0 == ']' => node.push_one(build_tree(dir, map, forward)),
        '[' if node.element.0 == ']' => node.push(build_tree(dir, map, (forward.0, forward.1)), build_tree(dir, map, (forward.0, forward.1 + 1))),
        ']' if node.element.0 == '[' => node.push(build_tree(dir, map, (forward.0, forward.1 - 1)), build_tree(dir, map, (forward.0, forward.1))),
        _ => panic!()
    }

    node
}

fn tree_move(dir: &Direction, map: &mut Vec<Vec<char>>, coord: (usize, usize)) -> (usize, usize) {
    let mut bt = build_tree(dir, map, coord);



    todo!()
}

#[allow(dead_code)]
pub fn part_1() -> Result<usize, Box<dyn Error>> {
    let (mut map, directions) = file_to_iter_chars_than_lines("src/inputs/input_15.txt", &mut parse_line)?;
    let n = map.len();

    let mut robot = (0, 0);
    for i in 0..n {
        for j in 0..n {
            if map[i][j] == '@' {
                robot = (i, j);
                break;
            }
        }
        if robot != (0, 0) { break; }
    }

    for d in directions.into_iter() {
        robot = recursive_move(&d, &mut map, robot);
    }

    let mut res = 0;
    for i in 0..n {
        for j in 0..n {
            if map[i][j] == 'O' { res += 100 * i + j; }
        }
    }

    Ok(res)
}

#[allow(dead_code)]
pub fn part_2() -> Result<usize, Box<dyn Error>> {
    let (mut map, directions) = file_to_iter_chars_than_lines("src/inputs/input_15.txt", &mut parse_line)?;
    let m = map.len();

    let mut new_map = Vec::new();
    let mut robot = (0, 0);
    for i in 0..m {
        new_map.push(Vec::new());
        let last = new_map.last_mut().ok_or("")?;
        for j in 0..m {
            match map[i][j] {
                '#' => { last.push('#'); last.push('#'); }
                'O' => { last.push('['); last.push(']'); }
                '•' => { last.push('.'); last.push('.'); }
                '@' => { last.push('@'); last.push('.'); robot = (i, j * 2) }
                _ => return Err(Box::from(""))
            }
        }
    }
    let n = new_map.len();

    for d in directions.into_iter().enumerate() {
        println!("Move {:?}:", (d.0, <Direction as Into<char>>::into(d.1.clone())));
        for tiles in map.clone() { println!("{}", tiles.iter().collect::<String>()) }
        println!();

        match d.1 {
            Direction::Right | Direction::Left => robot = recursive_move(&d.1, &mut map, robot),
            Direction::Up | Direction::Down => {
                //if let Some(r) = recursive_move_2(&d.1, &mut map, robot) { robot = r; }
            },
        }
    }

    let mut res = 0;
    for i in 0..n {
        for j in 0..n {
            if map[i][j] == '[' { res += 100 * i + j; }
        }
    }

    Ok(res)
}