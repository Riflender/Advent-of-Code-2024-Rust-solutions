use std::collections::{HashMap, HashSet};
use std::error::Error;
use core::iter::Iterator;
use crate::utils::{iterate_on_lines};

fn parse_line(line: &str) -> Result<(String, String), Box<dyn Error>> {
    let mut parts = line.split("-");
    let first = parts.next().ok_or("Pas assez de parties dans la chaîne")?.to_string();
    let second = parts.next().ok_or("Pas assez de parties dans la chaîne")?.to_string();

    Ok((first, second))
}

#[allow(dead_code)]
pub fn part_1() -> Result<usize, Box<dyn Error>> {
    let lines = iterate_on_lines("src/inputs/input_23.txt", parse_line)?;
    /*
    let lines = { vec![
        (String::from("kh"), String::from("tc")),
        (String::from("qp"), String::from("kh")),
        (String::from("de"), String::from("cg")),
        (String::from("ka"), String::from("co")),
        (String::from("yn"), String::from("aq")),
        (String::from("qp"), String::from("ub")),
        (String::from("cg"), String::from("tb")),
        (String::from("vc"), String::from("aq")),
        (String::from("tb"), String::from("ka")),
        (String::from("wh"), String::from("tc")),
        (String::from("yn"), String::from("cg")),
        (String::from("kh"), String::from("ub")),
        (String::from("ta"), String::from("co")),
        (String::from("de"), String::from("co")),
        (String::from("tc"), String::from("td")),
        (String::from("tb"), String::from("wq")),
        (String::from("wh"), String::from("td")),
        (String::from("ta"), String::from("ka")),
        (String::from("td"), String::from("qp")),
        (String::from("aq"), String::from("cg")),
        (String::from("wq"), String::from("ub")),
        (String::from("ub"), String::from("vc")),
        (String::from("de"), String::from("ta")),
        (String::from("wq"), String::from("aq")),
        (String::from("wq"), String::from("vc")),
        (String::from("wh"), String::from("yn")),
        (String::from("ka"), String::from("de")),
        (String::from("kh"), String::from("ta")),
        (String::from("co"), String::from("tc")),
        (String::from("wh"), String::from("qp")),
        (String::from("tb"), String::from("vc")),
        (String::from("td"), String::from("yn")),
    ]};
    */

    let mut computers = HashMap::new();
    for (a, b) in lines {
        if !computers.contains_key(&a) {
            computers.insert(a.clone(), Vec::new());
        }
        if !computers.contains_key(&b) {
            computers.insert(b.clone(), Vec::new());
        }
        computers.get_mut(&a).unwrap().push(b.clone());
        computers.get_mut(&b).unwrap().push(a);
    }

    let mut sets_of_three = HashSet::new();
    for (first, v) in computers.clone() {
        for second in v.clone() {
            for third in computers.get(&second).unwrap() {
                if v.contains(&third) {
                    let mut tmp = [first.clone(), second.clone(), third.clone()];
                    tmp.sort();

                    sets_of_three.insert((tmp[0].to_owned(), tmp[1].to_owned(), tmp[2].to_owned()));
                }
            }
        }
    }

    let mut sets: Vec<(String, String, String)> = sets_of_three.into_iter().collect();
    let mut res = 0;
    sets.sort();
    for set in sets {
        if set.0.chars().next().unwrap() == 't' ||
            set.1.chars().next().unwrap() == 't' ||
            set.2.chars().next().unwrap() == 't' { res += 1; }
    }

    Ok(res)
}

fn n(g: &Vec<Vec<usize>>, v: usize) -> HashSet<usize> {
    let mut res = HashSet::new();
    for i in 0..g.len() {
        if g[v][i] == 1 { res.insert(i); }
    }
    res
}

fn bron_kerbosch(g: &Vec<Vec<usize>>, r: HashSet<usize>, mut p: HashSet<usize>, mut x: HashSet<usize>) -> HashSet<usize> {
    if p.len() == 0 && x.len() == 0 {
        return r;
    }

    let mut clique_max = HashSet::new();
    for v in p.clone() {
        let tmp = bron_kerbosch(
            g,
            r.union(&HashSet::from([v])).map(|x| x.clone()).collect(),
            p.intersection(&n(g, v)).map(|x| x.clone()).collect(),
            x.intersection(&n(g, v)).map(|x| x.clone()).collect()
        );
        p.remove(&v);
        x.insert(v);

        if tmp.len() > clique_max.len() { clique_max = tmp; }
    }

    clique_max
}

#[allow(dead_code)]
pub fn part_2() -> Result<String, Box<dyn Error>> {
    let lines = iterate_on_lines("src/inputs/input_23.txt", parse_line)?;
    /*
    let lines = { vec![
        (String::from("kh"), String::from("tc")),
        (String::from("qp"), String::from("kh")),
        (String::from("de"), String::from("cg")),
        (String::from("ka"), String::from("co")),
        (String::from("yn"), String::from("aq")),
        (String::from("qp"), String::from("ub")),
        (String::from("cg"), String::from("tb")),
        (String::from("vc"), String::from("aq")),
        (String::from("tb"), String::from("ka")),
        (String::from("wh"), String::from("tc")),
        (String::from("yn"), String::from("cg")),
        (String::from("kh"), String::from("ub")),
        (String::from("ta"), String::from("co")),
        (String::from("de"), String::from("co")),
        (String::from("tc"), String::from("td")),
        (String::from("tb"), String::from("wq")),
        (String::from("wh"), String::from("td")),
        (String::from("ta"), String::from("ka")),
        (String::from("td"), String::from("qp")),
        (String::from("aq"), String::from("cg")),
        (String::from("wq"), String::from("ub")),
        (String::from("ub"), String::from("vc")),
        (String::from("de"), String::from("ta")),
        (String::from("wq"), String::from("aq")),
        (String::from("wq"), String::from("vc")),
        (String::from("wh"), String::from("yn")),
        (String::from("ka"), String::from("de")),
        (String::from("kh"), String::from("ta")),
        (String::from("co"), String::from("tc")),
        (String::from("wh"), String::from("qp")),
        (String::from("tb"), String::from("vc")),
        (String::from("td"), String::from("yn")),
    ]};
    */

    let mut index_map = HashMap::new();
    let mut map_index = HashMap::new();
    let mut f = |x: &String| {
        if !index_map.contains_key(x) {
            index_map.insert(x.clone(), index_map.len());
            map_index.insert(map_index.len(), x.clone());
        }
    };

    for (a, b) in lines.clone() {
        f(&a);
        f(&b);
    }

    let mut computers = vec![vec![0; index_map.len()]; index_map.len()];
    for (a, b) in lines {
        computers[index_map[&a]][index_map[&b]] = 1;
        computers[index_map[&b]][index_map[&a]] = 1;
    }

    let clique_max = bron_kerbosch(
        &computers,
        HashSet::new(),
        HashSet::from_iter(0..computers.len()),
        HashSet::new(),
    );

    let mut v = Vec::new();
    for c in clique_max {
        v.push(map_index[&c].clone());
    }
    v.sort();

    Ok(v.join(","))
}