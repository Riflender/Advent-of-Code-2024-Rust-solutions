use std::error::Error;
use std::collections::{HashSet, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;
use crate::utils::{file_to_iter_chars_than_lines, Direction};

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



#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
struct BinaryTree<'a, T>
where T: Clone {
    left: Option<Box<BinaryTree<'a, T>>>,
    right: Option<Box<BinaryTree<'a, T>>>,
    prev: Option<&'a BinaryTree<'a, T>>,
    element: T,
    len: usize,
}

impl<'a, T: Clone + PartialEq + Debug> BinaryTree<'a, T> {
    fn new(element: T) -> Self {
        BinaryTree { left: None, right: None, prev: None, element, len: 0 }
    }

    fn push(&mut self, left: BinaryTree<'a, T>, right: BinaryTree<'a, T>) {
        self.left = Some(Box::new(left));
        self.right = Some(Box::new(right));

        if let Some(l) = &self.left {
            l.to_owned().prev = Some(self);
        }
        if let Some(l) = &self.right {
            l.to_owned().prev = Some(self);
        }
    }

    fn push_one(&mut self, elt: BinaryTree<'a, T>) {
        self.left = Some(Box::new(elt));
        if let Some(l) = &self.left {
            l.to_owned().prev = Some(self);
        }
    }

    #[allow(dead_code)]
    fn contains(self, x: &T) -> bool
    where T: PartialEq, {
        match (self.left, self.right) {
            (Some(l), Some(r)) => (*l).element == *x || (*r).element == *x,
            (Some(n), _) | (_, Some(n)) => (*n).element == *x,
            (_, _) => false
        }
    }

    fn breadth_first(&self) -> HashSet<T>
    where T: Hash + Eq, {
        let mut queue= VecDeque::new();
        let mut marked_queue = VecDeque::new();

        queue.push_back(self.clone());
        marked_queue.push_back(self.clone());

        while !queue.is_empty() {
            let tmp = queue.pop_front().unwrap();

            if let Some(l) = tmp.left {
                if !marked_queue.contains(&*l) {
                    queue.push_back((*l).clone());
                    marked_queue.push_back((*l).clone());
                }
            }
            if let Some(r) = tmp.right {
                if !marked_queue.contains(&*r) {
                    queue.push_back((*r).clone());
                    marked_queue.push_back((*r).clone());
                }
            }
        }

        // println!("{:?}", marked_queue);
        marked_queue.into_iter().map(|x| x.element).collect::<HashSet<T>>()
    }
}

impl BinaryTree<'_, (char, (usize, usize))> {
    fn wall_inside(self) -> bool {
        match (self.left, self.right) {
            (Some(l), Some(r)) => (*l).wall_inside() || (*r).wall_inside(),
            (Some(n), _) | (_, Some(n)) => (*n).wall_inside(),
            (_, _) => self.element.0 == '#'
        }
    }
}

fn build_tree<'a>(dir: &Direction, map: &mut Vec<Vec<char>>, coord: (usize, usize)) -> BinaryTree<'a, (char, (usize, usize))> {
    let mut node = BinaryTree::new((map[coord.0][coord.1], coord));
    if map[coord.0][coord.1] == '.' || map[coord.0][coord.1] == '#' { return node }
    let forward = forward(dir, &coord);
    match map[forward.0][forward.1] {
        '.' |'#'  => node.push_one(build_tree(dir, map, forward)),
        '[' if node.element.0 == '[' => node.push_one(build_tree(dir, map, forward)),
        ']' if node.element.0 == ']' => node.push_one(build_tree(dir, map, forward)),
        '[' if node.element.0 == ']' || node.element.0 == '@' => node.push(
            build_tree(dir, map, (forward.0, forward.1)),
            build_tree(dir, map, (forward.0, forward.1 + 1))
        ),
        ']' if node.element.0 == '[' || node.element.0 == '@' => node.push(
            build_tree(dir, map, (forward.0, forward.1 - 1)),
            build_tree(dir, map, (forward.0, forward.1))
        ),
        _ => panic!()
    }
    node
}

fn move_tree(dir: &Direction, map: &mut Vec<Vec<char>>, tree: HashSet<(char, (usize, usize))>) {
    let mut t = tree.into_iter().map(|x| x.1).collect::<Vec<(usize, usize)>>();
    match dir {
        Direction::Up => t.sort(),
        Direction::Down => t.sort_by(|x, y| y.cmp(x)),
        _ => panic!(),
    }

    for coord in t {
        if map[coord.0][coord.1] == '.' { continue; }
        let forward = forward(dir, &coord);
        map[forward.0][forward.1] = map[coord.0][coord.1];
        map[coord.0][coord.1] = '.';
    }
}

fn vertical_move(dir: &Direction, map: &mut Vec<Vec<char>>, coord: (usize, usize)) -> (usize, usize) {
    let bt = build_tree(dir, map, coord);
    if bt.clone().wall_inside() { return coord }

    let hs = bt.breadth_first();
    move_tree(dir, map, hs);

    forward(dir, &coord)
}

#[allow(dead_code)]
fn print_map(dir: &Direction, iteration: usize, map: &Vec<Vec<char>>, robot: &(usize, usize)) {
    print!("{}[2J", 27 as char);
    let c: char = dir.into();
    println!("Move {:?}:", (iteration, c));
    let mut clone = map.clone();
    clone[robot.0][robot.1] = c;
    for tiles in clone { println!("{}", tiles.iter().collect::<String>()) }
    println!();
}

#[allow(dead_code)]
pub fn part_2() -> Result<usize, Box<dyn Error>> {
    let (map, directions) = file_to_iter_chars_than_lines("src/inputs/input_15.txt", &mut parse_line)?;

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
                '.' => { last.push('.'); last.push('.'); }
                '@' => { last.push('@'); last.push('.'); robot = (i, j * 2) }
                _ => return Err(Box::from(""))
            }
        }
    }
    let n = new_map[0].len();

    for (_i, d) in directions.into_iter().enumerate() {
        // if _i >= 2237 { print_map(&d, _i, &new_map, &robot); }

        match d {
            Direction::Right | Direction::Left => robot = recursive_move(&d, &mut new_map, robot),
            Direction::Up | Direction::Down => robot = vertical_move(&d, &mut new_map, robot),
        }
    }

    let mut res = 0;
    for i in 0..m {
        for j in 0..n {
            if new_map[i][j] == '[' { res += 100 * i + j; }
        }
    }

    Ok(res)
}