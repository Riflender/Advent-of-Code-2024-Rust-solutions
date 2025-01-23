use std::cmp::Ordering;
use std::collections::HashMap;
use std::error::Error;
use crate::utils::{iterate_on_lines_2};

#[derive(Clone, PartialEq, Eq, PartialOrd)]
enum Gate {
    AND,
    OR,
    XOR,
}

fn parse_line_1(line: &str) -> Result<(String, bool), Box<dyn Error>> {
    let mut parts = line.split(": ");
    let first = parts.next().ok_or("Pas assez de parties dans la chaîne")?.to_string();
    let second = parts.next().ok_or("Pas assez de parties dans la chaîne")?.parse::<u8>()? != 0;

    Ok((first, second))
}

fn parse_line_2(line: &str) -> Result<(String, Gate, String, String), Box<dyn Error>> {
    let mut parts = line.split(" ");
    let first = parts.next().ok_or("Pas assez de parties dans la chaîne")?.to_string();
    let second = match parts.next().ok_or("Pas assez de parties dans la chaîne")? {
        "AND" => Gate::AND,
        "OR" => Gate::OR,
        "XOR" => Gate::XOR,
        &_ => panic!(),
    };
    let third = parts.next().ok_or("Pas assez de parties dans la chaîne")?.to_string();
    parts.next();
    let forth = parts.next().ok_or("Pas assez de parties dans la chaîne")?.to_string();

    Ok((first, second, third, forth))
}

fn complete_wires(wires: Vec<(String, bool)>, mut gates: Vec<(String, Gate, String, String)>) -> Vec<(String, bool)> {
    let mut wires_map = wires.clone().into_iter().collect::<HashMap<String, bool>>();
    loop {
        let mut i = 0;
        while i < gates.len() {
            let (g1, gate, g2, g3) = gates[i].clone();
            if !wires_map.contains_key(&g1) || !wires_map.contains_key(&g2) {
                i += 1;
                continue;
            }

            wires_map.insert(g3,match gate {
                Gate::AND => wires_map[&g1] & wires_map[&g2],
                Gate::OR => wires_map[&g1] | wires_map[&g2],
                Gate::XOR => wires_map[&g1] ^ wires_map[&g2],
            });

            gates.remove(i);
        }

        if gates.is_empty() { break; }
    }
    /*
    let mut wires_map_2 = wires.into_iter().map(|(x, y)| (x.clone(), Wire::new(x, y))).collect::<HashMap<String, Wire>>();
    loop {
        let mut i = 0;
        while i < gates.len() {
            let (g1, gate, g2, g3) = gates[i].clone();
            if !wires_map_2.contains_key(&g1) || !wires_map_2.contains_key(&g2) {
                i += 1;
                continue;
            }

            wires_map_2.insert(g3.clone(), Wire::new_with_previous(
                g3,
                match gate {
                    Gate::AND => wires_map_2[&g1].value & wires_map_2[&g2].value,
                    Gate::OR => wires_map_2[&g1].value | wires_map_2[&g2].value,
                    Gate::XOR => wires_map_2[&g1].value ^ wires_map_2[&g2].value,
                },
                &wires_map_2[&g1],
                &wires_map_2[&g2],
                gate
            ));

            gates.remove(i);
        }

        if gates.is_empty() { break; }
    }
    let mut wires_vec_2 = wires_map_2.into_iter().collect::<Vec<(String, Wire)>>();
    */

    let mut wires_vec = wires_map.into_iter().collect::<Vec<(String, bool)>>();
    wires_vec.sort();
    wires_vec
}

#[allow(dead_code)]
pub fn part_1() -> Result<usize, Box<dyn Error>> {
    let (wires, gates) = iterate_on_lines_2(
        "src/inputs/input_24.txt",
        parse_line_1,
        parse_line_2
    )?;
    /*
    let wires = vec![
        (String::from("x00"), true),
        (String::from("x01"), false),
        (String::from("x02"), true),
        (String::from("x03"), true),
        (String::from("x04"), false),
        (String::from("y00"), true),
        (String::from("y01"), true),
        (String::from("y02"), true),
        (String::from("y03"), true),
        (String::from("y04"), true),
    ];
    let mut gates = vec![
        (String::from("ntg"), Gate::XOR, String::from("fgs"), String::from("mjb")),
        (String::from("y02"), Gate::OR, String::from("x01"), String::from("tnw")),
        (String::from("kwq"), Gate::OR, String::from("kpj"), String::from("z05")),
        (String::from("x00"), Gate::OR, String::from("x03"), String::from("fst")),
        (String::from("tgd"), Gate::XOR, String::from("rvg"), String::from("z01")),
        (String::from("vdt"), Gate::OR, String::from("tnw"), String::from("bfw")),
        (String::from("bfw"), Gate::AND, String::from("frj"), String::from("z10")),
        (String::from("ffh"), Gate::OR, String::from("nrd"), String::from("bqk")),
        (String::from("y00"), Gate::AND, String::from("y03"), String::from("djm")),
        (String::from("y03"), Gate::OR, String::from("y00"), String::from("psh")),
        (String::from("bqk"), Gate::OR, String::from("frj"), String::from("z08")),
        (String::from("tnw"), Gate::OR, String::from("fst"), String::from("frj")),
        (String::from("gnj"), Gate::AND, String::from("tgd"), String::from("z11")),
        (String::from("bfw"), Gate::XOR, String::from("mjb"), String::from("z00")),
        (String::from("x03"), Gate::OR, String::from("x00"), String::from("vdt")),
        (String::from("gnj"), Gate::AND, String::from("wpb"), String::from("z02")),
        (String::from("x04"), Gate::AND, String::from("y00"), String::from("kjc")),
        (String::from("djm"), Gate::OR, String::from("pbm"), String::from("qhw")),
        (String::from("nrd"), Gate::AND, String::from("vdt"), String::from("hwm")),
        (String::from("kjc"), Gate::AND, String::from("fst"), String::from("rvg")),
        (String::from("y04"), Gate::OR, String::from("y02"), String::from("fgs")),
        (String::from("y01"), Gate::AND, String::from("x02"), String::from("pbm")),
        (String::from("ntg"), Gate::OR, String::from("kjc"), String::from("kwq")),
        (String::from("psh"), Gate::XOR, String::from("fgs"), String::from("tgd")),
        (String::from("qhw"), Gate::XOR, String::from("tgd"), String::from("z09")),
        (String::from("pbm"), Gate::OR, String::from("djm"), String::from("kpj")),
        (String::from("x03"), Gate::XOR, String::from("y03"), String::from("ffh")),
        (String::from("x00"), Gate::XOR, String::from("y04"), String::from("ntg")),
        (String::from("bfw"), Gate::OR, String::from("bqk"), String::from("z06")),
        (String::from("nrd"), Gate::XOR, String::from("fgs"), String::from("wpb")),
        (String::from("frj"), Gate::XOR, String::from("qhw"), String::from("z04")),
        (String::from("bqk"), Gate::OR, String::from("frj"), String::from("z07")),
        (String::from("y03"), Gate::OR, String::from("x01"), String::from("nrd")),
        (String::from("hwm"), Gate::AND, String::from("bqk"), String::from("z03")),
        (String::from("tgd"), Gate::XOR, String::from("rvg"), String::from("z12")),
        (String::from("tnw"), Gate::OR, String::from("pbm"), String::from("gnj")),
    ];
    */

    let wires_vec = complete_wires(wires, gates);
    let mut bin = String::new();

    for (k, v) in wires_vec.iter() { println!("{k}: {v}"); }
    for (k, v) in wires_vec.into_iter().rev() {
        if k.starts_with("z") { bin.push_str(&*(v as u8).to_string()) }
        else { break; }
    }

    let res = usize::from_str_radix(&*bin, 2)?;

    Ok(res)
}

#[allow(dead_code)]
#[derive(PartialEq, Eq, PartialOrd)]
struct Wire<'a> {
    name: String,
    value: bool,
    left: Option<&'a Wire<'a>>,
    right: Option<&'a Wire<'a>>,
    gate: Option<Gate>,
}

impl Ord for Wire<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

#[allow(dead_code)]
impl<'a> Wire<'a> {
    fn new(name: String, value: bool) -> Self {
        Wire {
            name,
            value,
            left: None,
            right: None,
            gate: None,
        }
    }

    fn new_with_previous<'b: 'a>(name: String, value: bool, left: &'b Wire, right: &'b Wire, gate: Gate) -> Self {
        Wire {
            name,
            value,
            left: Some(left),
            right: Some(right),
            gate: Some(gate),
        }
    }
}

#[allow(dead_code)]
pub fn part_2() -> Result<usize, Box<dyn Error>> {
    let (wires, gates) = iterate_on_lines_2(
        "src/inputs/input_24.txt",
        parse_line_1,
        parse_line_2
    )?;
    /*
    let wires = vec![
        (String::from("x00"), true),
        (String::from("x01"), false),
        (String::from("x02"), true),
        (String::from("x03"), true),
        (String::from("x04"), false),
        (String::from("y00"), true),
        (String::from("y01"), true),
        (String::from("y02"), true),
        (String::from("y03"), true),
        (String::from("y04"), true),
    ];
    let mut gates = vec![
        (String::from("ntg"), Gate::XOR, String::from("fgs"), String::from("mjb")),
        (String::from("y02"), Gate::OR, String::from("x01"), String::from("tnw")),
        (String::from("kwq"), Gate::OR, String::from("kpj"), String::from("z05")),
        (String::from("x00"), Gate::OR, String::from("x03"), String::from("fst")),
        (String::from("tgd"), Gate::XOR, String::from("rvg"), String::from("z01")),
        (String::from("vdt"), Gate::OR, String::from("tnw"), String::from("bfw")),
        (String::from("bfw"), Gate::AND, String::from("frj"), String::from("z10")),
        (String::from("ffh"), Gate::OR, String::from("nrd"), String::from("bqk")),
        (String::from("y00"), Gate::AND, String::from("y03"), String::from("djm")),
        (String::from("y03"), Gate::OR, String::from("y00"), String::from("psh")),
        (String::from("bqk"), Gate::OR, String::from("frj"), String::from("z08")),
        (String::from("tnw"), Gate::OR, String::from("fst"), String::from("frj")),
        (String::from("gnj"), Gate::AND, String::from("tgd"), String::from("z11")),
        (String::from("bfw"), Gate::XOR, String::from("mjb"), String::from("z00")),
        (String::from("x03"), Gate::OR, String::from("x00"), String::from("vdt")),
        (String::from("gnj"), Gate::AND, String::from("wpb"), String::from("z02")),
        (String::from("x04"), Gate::AND, String::from("y00"), String::from("kjc")),
        (String::from("djm"), Gate::OR, String::from("pbm"), String::from("qhw")),
        (String::from("nrd"), Gate::AND, String::from("vdt"), String::from("hwm")),
        (String::from("kjc"), Gate::AND, String::from("fst"), String::from("rvg")),
        (String::from("y04"), Gate::OR, String::from("y02"), String::from("fgs")),
        (String::from("y01"), Gate::AND, String::from("x02"), String::from("pbm")),
        (String::from("ntg"), Gate::OR, String::from("kjc"), String::from("kwq")),
        (String::from("psh"), Gate::XOR, String::from("fgs"), String::from("tgd")),
        (String::from("qhw"), Gate::XOR, String::from("tgd"), String::from("z09")),
        (String::from("pbm"), Gate::OR, String::from("djm"), String::from("kpj")),
        (String::from("x03"), Gate::XOR, String::from("y03"), String::from("ffh")),
        (String::from("x00"), Gate::XOR, String::from("y04"), String::from("ntg")),
        (String::from("bfw"), Gate::OR, String::from("bqk"), String::from("z06")),
        (String::from("nrd"), Gate::XOR, String::from("fgs"), String::from("wpb")),
        (String::from("frj"), Gate::XOR, String::from("qhw"), String::from("z04")),
        (String::from("bqk"), Gate::OR, String::from("frj"), String::from("z07")),
        (String::from("y03"), Gate::OR, String::from("x01"), String::from("nrd")),
        (String::from("hwm"), Gate::AND, String::from("bqk"), String::from("z03")),
        (String::from("tgd"), Gate::XOR, String::from("rvg"), String::from("z12")),
        (String::from("tnw"), Gate::OR, String::from("pbm"), String::from("gnj")),
    ];
    */

    let wires_vec = complete_wires(wires, gates);
    let mut bin = String::new();

    for (k, v) in wires_vec.iter() { println!("{k}: {v}"); }
    for (k, v) in wires_vec.into_iter().rev() {
        if k.starts_with("z") { bin.push_str(&*(v as u8).to_string()) }
        else { break; }
    }

    let res = usize::from_str_radix(&*bin, 2)?;

    Ok(res)
}