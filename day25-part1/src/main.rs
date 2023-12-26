// Solving this problem seems to be hard problem. However, a technique called
// spectral partitioning can provide a number of sane candidate partitions that
// balance separation of groups whilst minimising "cuts". We search for a
// grouping that results in 3 cuts out of the candidate partitions

use std::fs;
use std::str;
use std::collections::{HashMap, HashSet};
use nalgebra_sparse::{CooMatrix, CsrMatrix};
use svdlibrs::svd;

fn parse_netlist(input: &str) -> HashMap<String, Vec<String>> {
    let mut netlist = HashMap::new();
    for l in input.lines() {
        let (comp, rest) = l.split_once(": ").unwrap();
        let connected: Vec<String> = rest.split(" ").map(|x| x.to_string()).collect();
        netlist.insert(comp.to_string(), connected);
    }
    netlist
}

fn partial_netlist_to_full(netlist: HashMap<String, Vec<String>>) -> HashMap<String, HashSet<String>> {

    // Determine total set of parts
    let mut parts = HashSet::new();
    for (comp, conn_list) in netlist.iter() {
        parts.insert(comp.clone());
        for c in conn_list {
            parts.insert(c.clone());
        }
    }

    let mut new_netlist = HashMap::new();

    // Initialise graph
    for p in &parts {
        new_netlist.insert(p.clone(), HashSet::new());
    }

    for comp1 in parts.iter() {
        if let Some(conn_list1) = netlist.get(comp1) {
            for c in conn_list1 {
                new_netlist.get_mut(comp1).unwrap().insert(c.clone());
            }
        }

        for (comp2, conn_list2) in netlist.iter() {
            if conn_list2.contains(comp1) {
                new_netlist.get_mut(comp1).unwrap().insert(comp2.clone());
            }
        }
    }

    new_netlist
}

fn netlist_to_matrix(netlist: &HashMap<String, HashSet<String>>, mapping: &Vec<String>) -> CsrMatrix<f64> {
    let len = netlist.len();
    let mut coo = CooMatrix::<f64>::new(len, len);
    for (p1, conns) in netlist {
        let row = mapping.iter().position(|s| s == p1).unwrap();
        for p2 in conns {
            let col = mapping.iter().position(|s| s == p2).unwrap();
            coo.push(row, col, -1.0);
            coo.push(row, row, 1.0);
        }
    }
    CsrMatrix::from(&coo)
}

fn calc_candidate_groups(netlist: &HashMap<String, HashSet<String>>, num_grps: usize) -> Vec<Vec<String>> {
    let mapping = netlist.keys().cloned().collect();
    let mat = netlist_to_matrix(&netlist, &mapping);
    let svd = svd(&mat).unwrap();
    let ut = svd.ut.t();

    let mut groups = vec![];

    for i in 0..num_grps {
        let v = ut.column(svd.d - 1 - i);
        let group = v
            .indexed_iter()
            .filter(|(_, v)| **v > 0.0)
            .map(|(i, _)| (&mapping[i]).clone())
            .collect();
        groups.push(group);
    }

    groups
}

fn calc_cuts(netlist: &HashMap<String, HashSet<String>>, group: &Vec<String>) -> usize {
    let group1 = group;

    let mut total_cuts = 0;
    for p1 in group1 {
        let conns = netlist.get(p1).unwrap();
        for p2 in conns {
            if !group1.contains(p2) {
                total_cuts += 1;
            }
        }
    }

    total_cuts
}

fn main() {
    let contents = fs::read_to_string("input/input.txt").unwrap();
    let contents = contents.trim();
    let netlist = parse_netlist(contents);
    let netlist = partial_netlist_to_full(netlist);

    let groups = calc_candidate_groups(&netlist, 10);

    for g in groups {
        let cuts = calc_cuts(&netlist, &g);
        if cuts == 3{
            println!("answer = {}", g.len() * (netlist.len() - g.len()));
        }
    }
}
