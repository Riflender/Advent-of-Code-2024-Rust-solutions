use std::error::Error;
use crate::utils::iterate_on_lines;

#[allow(dead_code)]
fn print_tiles(robots: Vec<(isize, isize, isize, isize)>, width: isize, height: isize) {
    let mut tiles: Vec<Vec<Option<usize>>> = vec![vec![None; width as usize]; height as usize];

    for (px, py, _, _) in robots {
        let (u_px, u_py) = (px as usize, py as usize);
        match tiles[u_py][u_px] {
            Some(s) => tiles[u_py][u_px] = Some(s + 1),
            None => tiles[u_py][u_px] = Some(1),
        }
    }

    for t in tiles {
        println!("{}", t
            .into_iter()
            .map(|y| match y {
                Some(s) => s.to_string(),
                None => String::from(".")
            })
            .collect::<Vec<String>>()
            .join(""));
    }
}

fn parse_line(line: &str) -> Result<(isize, isize, isize, isize), Box<dyn Error>> {
    let v = line
        .split(' ')
        .map(|x| x.split(',').collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    Ok((
        v[0][0][2..].parse()?,
        v[0][1].parse()?,
        v[1][0][2..].parse()?,
        v[1][1].parse()?
    ))
}

#[allow(dead_code)]
pub fn part_1() -> Result<usize, Box<dyn Error>> {
    let robots = iterate_on_lines("src/inputs/input_14.txt", &parse_line)?;
    let width = 101;
    let height = 103;

    let mut q = (0, 0, 0, 0);
    for (px, py, vx, vy) in robots {
        match ((px + vx * 8270).rem_euclid(width), (py + vy * 8270).rem_euclid(height)) {
            (x, y) if x < 50 && y < 51 => q.0 += 1,
            (x, y) if x > 50 && y < 51 => q.1 += 1,
            (x, y) if x < 50 && y > 51 => q.2 += 1,
            (x, y) if x > 50 && y > 51 => q.3 += 1,
            (_, _) => continue,
        }
    }

    Ok(q.0 * q.1 * q.2 * q.3)
}

#[allow(dead_code)]
pub fn part_2() -> Result<usize, Box<dyn Error>> {
    let mut robots = iterate_on_lines("src/inputs/input_14.txt", &parse_line)?;
    let width = 101;
    let height = 103;

    let mut i = 0;
    loop {
        let mut tiles = vec![vec!["."; width as usize]; height as usize];
        for (px, py, _, _) in robots.clone() {
            let (u_px, u_py) = (px as usize, py as usize);
            tiles[u_py][u_px] = "#";
        }

        for t in tiles {
            if t.join("").contains("###############################") { return Ok(i); }
        }

        let _ = robots.iter_mut().for_each(|x| {
            (*x).0 = (x.0 + x.2).rem_euclid(width);
            (*x).1 = (x.1 + x.3).rem_euclid(height);
        });

        i += 1;
    }
}

/*
###############################
#.............................#
#.............................#
#.............................#
#.............................#
#..............#..............#
#.............###.............#
#............#####............#
#...........#######...........#
#..........#########..........#
#............#####............#
#...........#######...........#
#..........#########..........#
#.........###########.........#
#........#############........#
#..........#########..........#
#.........###########.........#
#........#############........#
#.......###############.......#
#......#################......#
#........#############........#
#.......###############.......#
#......#################......#
#.....###################.....#
#....#####################....#
#.............###.............#
#.............###.............#
#.............###.............#
#.............................#
#.............................#
#.............................#
#.............................#
###############################
*/
