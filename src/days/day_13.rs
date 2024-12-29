use std::error::Error;
// use simplex::{Simplex, SimplexConstraint, SimplexOutput};
use crate::utils::iterate_on_lines_batch;

#[allow(dead_code)]
struct Coords<T> {
    x: T,
    y: T,
}

#[allow(dead_code)]
struct Machine {
    button_a: Coords<u8>,
    button_b: Coords<u8>,
    prize: Coords<u32>,
}

fn parse_batch(batch: &[String]) -> Result<Machine, Box<dyn Error>> {
    let v = batch[2].split(" ").collect::<Vec<&str>>();

    Ok(Machine {
        button_a: Coords { x: batch[0][12..14].parse()?, y: batch[0][18..20].parse()? },
        button_b: Coords { x: batch[1][12..14].parse()?, y: batch[1][18..20].parse()? },
        prize: Coords { x: v[1][2..v[1].len() - 1].parse()?, y: v[2][2..v[2].len()].parse()? },
    })
}

#[allow(dead_code)]
pub fn part_1() -> Result<usize, Box<dyn Error>> {
    let _batch = iterate_on_lines_batch("src/inputs/input_13.txt", 4, &mut parse_batch)?;

    /*
    https://cbom.atozmath.com/CBOM/Simplex.aspx?q=sm

    let mut problem = Simplex::minimize(&vec![3.0, 1.0]).with(vec![
        SimplexConstraint::Equal(vec![94.0, 22.0], 8400.0),
        SimplexConstraint::Equal(vec![34.0, 67.0], 5400.0),
    ])?;
    match problem.solve() {
        SimplexOutput::UniqueOptimum(_) => println!("UniqueOptimum"),
        SimplexOutput::MultipleOptimum(_) => println!("MultipleOptimum"),
        SimplexOutput::InfiniteSolution => println!("InfiniteSolution"),
        SimplexOutput::NoSolution => println!("NoSolution"),
    }

    let mut simplex = problem.solve();
    println!("{:?}", problem);
    */

    Ok(0)
}

#[allow(dead_code)]
pub fn part_2() -> Result<usize, Box<dyn Error>> {
    let _batch = iterate_on_lines_batch("src/inputs/input_13.txt", 4, &mut parse_batch)?;



    Ok(0)
}