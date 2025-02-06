use std::error::Error;
use std::fs::{remove_file, write};
use std::process::Stdio;
use crate::utils::{iterate_on_lines_batch, read_lines};

#[allow(dead_code)]
struct Coords {
    x: i64,
    y: i64,
}

#[allow(dead_code)]
struct Machine {
    a: Coords,
    b: Coords,
    prize: Coords,
}

fn parse_batch(batch: &[String]) -> Result<Machine, Box<dyn Error>> {
    let v = batch[2].split(" ").collect::<Vec<&str>>();

    Ok(Machine {
        a: Coords { x: batch[0][12..14].parse()?, y: batch[0][18..20].parse()? },
        b: Coords { x: batch[1][12..14].parse()?, y: batch[1][18..20].parse()? },
        prize: Coords { x: v[1][2..v[1].len() - 1].parse()?, y: v[2][2..v[2].len()].parse()? },
    })
}

#[allow(dead_code)]
pub fn part_1() -> Result<usize, Box<dyn Error>> {
    let batch = iterate_on_lines_batch("src/inputs/input_13.txt", 4, &mut parse_batch)?;
    /*
    let batch = vec![
        Machine { a: Coords { x: 94, y: 34 }, b: Coords { x: 22, y: 67 }, prize: Coords { x: 8400, y: 5400 } },
        Machine { a: Coords { x: 26, y: 66 }, b: Coords { x: 67, y: 21 }, prize: Coords { x: 12748, y: 12176 } },
        Machine { a: Coords { x: 17, y: 86 }, b: Coords { x: 84, y: 37 }, prize: Coords { x: 7870, y: 6450 } },
        Machine { a: Coords { x: 69, y: 23 }, b: Coords { x: 27, y: 71 }, prize: Coords { x: 18641, y: 10279 } }
    ];
    */

    let mut res = 0;
    for machine in batch {
        let t = format!(
            "Minimize\nz: 3x + y\nSubject To\na1: {}x + {}y = {}\na2: {}x + {}y = {}\nGeneral\nx\ny\nEnd\n",
            machine.a.x, machine.b.x, machine.prize.x,
            machine.a.y, machine.b.y, machine.prize.y
        );

        write("day_13.lp", t)?;
        std::process::Command::new("glpsol")
            .args(["--lp", "day_13.lp", "--output", "day_13.sol"])
            .stdout(Stdio::null())
            .status()?;
        let lines = read_lines("day_13.sol")?;

        let sol = &lines.flatten().collect::<Vec<String>>()[4..6];
        if sol[0].split_whitespace().last().unwrap() == "OPTIMAL" {
            let z = &sol[1].split_whitespace().collect::<Vec<&str>>()[3..4];
            res += z[0].parse::<usize>()?;
        }
    }

    remove_file("day_13.lp")?;
    remove_file("day_13.sol")?;

    Ok(res)
}

#[allow(dead_code)]
pub fn part_2() -> Result<i64, Box<dyn Error>> {
    let batch = iterate_on_lines_batch("src/inputs/input_13.txt", 4, &mut parse_batch)?;
    /*
    let batch = vec![
        Machine { a: Coords { x: 94, y: 34 }, b: Coords { x: 22, y: 67 }, prize: Coords { x: 8400, y: 5400 } },
        Machine { a: Coords { x: 26, y: 66 }, b: Coords { x: 67, y: 21 }, prize: Coords { x: 12748, y: 12176 } },
        Machine { a: Coords { x: 17, y: 86 }, b: Coords { x: 84, y: 37 }, prize: Coords { x: 7870, y: 6450 } },
        Machine { a: Coords { x: 69, y: 23 }, b: Coords { x: 27, y: 71 }, prize: Coords { x: 18641, y: 10279 } }
    ];
    */

    let mut res = 0;
    for machine in batch {
        let d = machine.a.x * machine.b.y - machine.a.y * machine.b.x;
        let d_x = (machine.prize.x + 10000000000000) * machine.b.y - (machine.prize.y + 10000000000000) * machine.b.x;
        let d_y = machine.a.x * (machine.prize.y + 10000000000000) - machine.a.y * (machine.prize.x + 10000000000000);
        let a = d_x / d;
        let b = d_y / d;

        if d_x % d != 0 || d_y % d != 0 { continue; }

        res += 3 * a + b;
    }

    Ok(res)
}