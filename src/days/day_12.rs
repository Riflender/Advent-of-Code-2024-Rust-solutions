use std::error::Error;
use crate::utils::file_to_chars;

struct Plot {
    label: char,
    nb_corners: usize,
    marked: bool,
}

impl Plot {
    fn new(label: char) -> Self {
        Self { label, nb_corners: 0, marked: false }
    }
}

struct Region {
    label: char,
    area: usize,
    perimeter: usize,
    plots: Vec<(usize, usize)>
}

impl Region {
    fn new(label: char) -> Self {
        Self { label, area: 0, perimeter: 0, plots: vec![] }
    }
}

fn to_plots(mat: Vec<Vec<char>>) -> Vec<Vec<Plot>> {
    mat.into_iter()
        .map(|x| x.into_iter().map(|y| Plot::new(y)).collect::<Vec<Plot>>())
        .collect::<Vec<Vec<Plot>>>()
}

fn to_regions(mat: Vec<Vec<char>>) -> Vec<Region> {
    let mut plots = to_plots(mat);
    let n = plots.len();
    let mut regions = Vec::new();

    for i in 0..n {
        for j in 0..n {
            if !plots[i][j].marked {
                let mut region = Region::new(plots[i][j].label);
                depth_first(&mut plots, i, j, &mut region);
                regions.push(region);
            }
        }
    }

    regions
}

fn depth_first(plots: &mut Vec<Vec<Plot>>, i: usize, j: usize, region: &mut Region) {
    plots[i][j].marked = true;
    region.area += 1;
    region.plots.push((i, j));

    if i > 0 {
        if plots[i - 1][j].label == region.label {
            if !plots[i - 1][j].marked { depth_first(plots, i - 1, j, region); }
        } else { region.perimeter += 1; }
    } else { region.perimeter += 1; }

    if j > 0 {
        if plots[i][j - 1].label == region.label {
            if !plots[i][j - 1].marked { depth_first(plots, i, j - 1, region); }
        } else { region.perimeter += 1; }
    } else { region.perimeter += 1; }

    if (i + 1) < plots.len() {
        if plots[i + 1][j].label == region.label {
            if !plots[i + 1][j].marked { depth_first(plots, i + 1, j, region); }
        } else { region.perimeter += 1; }
    } else { region.perimeter += 1; }

    if (j + 1) < plots.len() {
        if plots[i][j + 1].label == region.label {
            if !plots[i][j + 1].marked { depth_first(plots, i, j + 1, region); }
        } else { region.perimeter += 1; }
    } else { region.perimeter += 1; }
}

fn count_corners(plots: &mut Vec<Vec<Plot>>) {
    let n = plots.len();
    for i in 0..n {
        for j in 0..n {
            let (u, r, d, l) = (i > 0, (j + 1) < n, (i + 1) < n, j > 0);

            match (u, r) {
                (true, true) => if plots[i][j].label != plots[i - 1][j].label && plots[i][j].label != plots[i][j + 1].label {
                    plots[i][j].nb_corners += 1;
                    if plots[i - 1][j].label == plots[i - 1][j + 1].label && plots[i][j + 1].label == plots[i - 1][j + 1].label {
                        plots[i - 1][j + 1].nb_corners += 1;
                    }
                }
                (true, false) => if plots[i][j].label != plots[i - 1][j].label { plots[i][j].nb_corners += 1; }
                (false, true) => if plots[i][j].label != plots[i][j + 1].label { plots[i][j].nb_corners += 1; }
                (false, false) => plots[i][j].nb_corners += 1,
            }

            match (r, d) {
                (true, true) => if plots[i][j].label != plots[i][j + 1].label && plots[i][j].label != plots[i + 1][j].label {
                    plots[i][j].nb_corners += 1;
                    if plots[i + 1][j].label == plots[i + 1][j + 1].label && plots[i][j + 1].label == plots[i + 1][j + 1].label {
                        plots[i + 1][j + 1].nb_corners += 1;
                    }
                }
                (true, false) => if plots[i][j].label != plots[i][j + 1].label { plots[i][j].nb_corners += 1; }
                (false, true) => if plots[i][j].label != plots[i + 1][j].label { plots[i][j].nb_corners += 1; }
                (false, false) => plots[i][j].nb_corners += 1,
            }

            match (d, l) {
                (true, true) => if plots[i][j].label != plots[i + 1][j].label && plots[i][j].label != plots[i][j - 1].label {
                    plots[i][j].nb_corners += 1;
                    if plots[i + 1][j].label == plots[i + 1][j - 1].label && plots[i][j - 1].label == plots[i + 1][j - 1].label {
                        plots[i + 1][j - 1].nb_corners += 1;
                    }
                }
                (true, false) => if plots[i][j].label != plots[i + 1][j].label { plots[i][j].nb_corners += 1; }
                (false, true) => if plots[i][j].label != plots[i][j - 1].label { plots[i][j].nb_corners += 1; }
                (false, false) => plots[i][j].nb_corners += 1,
            }

            match (l, u) {
                (true, true) => if plots[i][j].label != plots[i][j - 1].label && plots[i][j].label != plots[i - 1][j].label {
                    plots[i][j].nb_corners += 1;
                    if plots[i][j - 1].label == plots[i - 1][j - 1].label && plots[i - 1][j].label == plots[i - 1][j - 1].label {
                        plots[i - 1][j - 1].nb_corners += 1;
                    }
                }
                (true, false) => if plots[i][j].label != plots[i][j - 1].label { plots[i][j].nb_corners += 1; }
                (false, true) => if plots[i][j].label != plots[i - 1][j].label { plots[i][j].nb_corners += 1; }
                (false, false) => plots[i][j].nb_corners += 1,
            }
        }
    }
}

#[allow(dead_code)]
pub fn part_1() -> Result<usize, Box<dyn Error>> {
    let lines = file_to_chars("src/inputs/input_12.txt")?;
    /*
    let data = vec![
        vec!['R', 'R', 'R', 'R', 'I', 'I', 'C', 'C', 'F', 'F'],
        vec!['R', 'R', 'R', 'R', 'I', 'I', 'C', 'C', 'C', 'F'],
        vec!['V', 'V', 'R', 'R', 'R', 'C', 'C', 'F', 'F', 'F'],
        vec!['V', 'V', 'R', 'C', 'C', 'C', 'J', 'F', 'F', 'F'],
        vec!['V', 'V', 'V', 'V', 'C', 'J', 'J', 'C', 'F', 'E'],
        vec!['V', 'V', 'I', 'V', 'C', 'C', 'J', 'J', 'E', 'E'],
        vec!['V', 'V', 'I', 'I', 'I', 'C', 'J', 'J', 'E', 'E'],
        vec!['M', 'I', 'I', 'I', 'I', 'I', 'J', 'J', 'E', 'E'],
        vec!['M', 'I', 'I', 'I', 'S', 'I', 'J', 'E', 'E', 'E'],
        vec!['M', 'M', 'M', 'I', 'S', 'S', 'J', 'E', 'E', 'E']
    ];
    */

    let regions = to_regions(lines);

    let mut res = 0;
    for region in regions {
        res += region.area * region.perimeter;
    }

    Ok(res)
}

#[allow(dead_code)]
pub fn part_2() -> Result<usize, Box<dyn Error>> {
    let lines = file_to_chars("src/inputs/input_12.txt")?;
    /*
    let lines = vec![
        vec!['R', 'R', 'R', 'R', 'I', 'I', 'C', 'C', 'F', 'F'],
        vec!['R', 'R', 'R', 'R', 'I', 'I', 'C', 'C', 'C', 'F'],
        vec!['V', 'V', 'R', 'R', 'R', 'C', 'C', 'F', 'F', 'F'],
        vec!['V', 'V', 'R', 'C', 'C', 'C', 'J', 'F', 'F', 'F'],
        vec!['V', 'V', 'V', 'V', 'C', 'J', 'J', 'C', 'F', 'E'],
        vec!['V', 'V', 'I', 'V', 'C', 'C', 'J', 'J', 'E', 'E'],
        vec!['V', 'V', 'I', 'I', 'I', 'C', 'J', 'J', 'E', 'E'],
        vec!['M', 'I', 'I', 'I', 'I', 'I', 'J', 'J', 'E', 'E'],
        vec!['M', 'I', 'I', 'I', 'S', 'I', 'J', 'E', 'E', 'E'],
        vec!['M', 'M', 'M', 'I', 'S', 'S', 'J', 'E', 'E', 'E']
    ];
    */

    let regions = to_regions(lines.clone());
    let mut plots = to_plots(lines);
    count_corners(&mut plots);

    let mut res = 0;
    for region in regions {
        let mut corners = 0;
        for (i, j) in region.plots { corners += plots[i][j].nb_corners; }

        res += region.area * corners;
    }

    Ok(res)
}