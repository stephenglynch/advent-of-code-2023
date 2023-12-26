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

fn netlist_to_vector(netlist: HashMap<String, Vec<String>>) -> (HashMap<usize, Vec<usize>>, Vec<String>) {

    // Determine total set of parts
    let mut parts = HashSet::new();
    for (comp, conn_list) in netlist.iter() {
        parts.insert(comp.clone());
        for c in conn_list {
            parts.insert(c.clone());
        }
    }

    let parts_map: HashMap<String, usize> = parts.iter().enumerate().map(|(i, k)| (k.clone(), i)).collect();
    let mut graph = HashMap::new();

    // Initialise graph
    for p in &parts {
        let i = *parts_map.get(p).unwrap();
        graph.insert(i, vec![]);
    }

    for comp1 in parts.iter() {
        let row = *parts_map.get(comp1).unwrap();
        if let Some(conn_list1) = netlist.get(comp1) {
            for c in conn_list1 {
                let col = *parts_map.get(c).unwrap();
                graph.get_mut(&row).unwrap().push(col);
            }
        }

        for (comp2, conn_list2) in netlist.iter() {
            let col = *parts_map.get(comp2).unwrap();

            if conn_list2.contains(comp1) {
                graph.get_mut(&row).unwrap().push(col);
            }
        }
    }

    let designators = parts.into_iter().collect();

    (graph, designators)
}

fn generate_matrix(graph: &HashMap<usize, Vec<usize>>) -> CsrMatrix<f64> {
    let len = graph.len();
    let mut coo = CooMatrix::<f64>::new(len, len);
    for (row, cols) in graph {
        for col in cols {
            coo.push(*row, *col, -1.0);
            coo.push(*col, *row, -1.0);
            coo.push(*row, *row, 1.0);
        }
    }
    CsrMatrix::from(&coo)
}

fn calc_cuts(graph: &HashMap<usize, Vec<usize>>, group: &Vec<usize>) -> usize {
    let group1 = group;
    let group2: Vec<usize> = graph.keys().copied().filter(|p| !group1.contains(p)).collect();

    let mut total_cuts = 0;
    for p1 in group1 {
        let conns = graph.get(p1).unwrap();
        for p2 in conns {
            if !group1.contains(p2) {
                total_cuts += 1;
            }
        }
    }

    total_cuts
}

fn print_group(group: &Vec<usize>, desig: &Vec<String>) {
    let mut v: Vec<String> = group.iter().map(|r| (&desig[*r]).to_string()).collect();
    v.sort();
    println!("group:\n{:?}", v);
}

fn main() {
    let contents = fs::read_to_string("input/example.txt").unwrap();
    let contents = contents.trim();
    let netlist = parse_netlist(contents);
    
    let (graph, desig) = netlist_to_vector(netlist);
    println!("graph:\n{:?}", graph);
    let len = graph.len();
    let mat = generate_matrix(&graph);

    println!("mat = {}", mat.ncols());
    let svd = svd(&mat).unwrap();
    println!("svd.d = {}", svd.d);
    // println!("svd.s:\n{:?}", svd.s);
    println!("svd.ut:\n{}", svd.ut.t().column(13).len());

    let ut = svd.ut.t();

    println!("eigenvalues:\n{}", svd.s);
    println!();

    let v = ut.column(svd.d - 1);
    let group = v.indexed_iter().filter(|(_, v)| **v > 0.0).map(|(i, _)| i).collect();
    print_group(&group, &desig);
    let cuts = calc_cuts(&graph, &group);
    let g1_len = v.iter().filter(|x| (**x) > 0.0).count();
    let g2_len = len - g1_len;
    println!("cuts = {}", cuts);
    println!("answer = {}", g1_len * g2_len);
    println!();

    let v = ut.column(svd.d - 2);
    let group = v.indexed_iter().filter(|(_, v)| **v > 0.0).map(|(i, _)| i).collect();
    print_group(&group, &desig);
    let cuts = calc_cuts(&graph, &group);
    let g1_len = v.iter().filter(|x| (**x) > 0.0).count();
    let g2_len = len - g1_len;
    println!("cuts = {}", cuts);
    println!("answer = {}", g1_len * g2_len);
    println!();

    let v = ut.column(svd.d - 3);
    let group = v.indexed_iter().filter(|(_, v)| **v > 0.0).map(|(i, _)| i).collect();
    print_group(&group, &desig);
    let cuts = calc_cuts(&graph, &group);
    let g1_len = v.iter().filter(|x| (**x) > 0.0).count();
    let g2_len = len - g1_len;
    println!("cuts = {}", cuts);
    println!("answer = {}", g1_len * g2_len);

    // let eig = lap_mat.symmetric_eigen();
    // println!("eigenvalues:\n{}", eig.eigenvalues);

    // for i in 0..len {
    //     println!("eigenvector {}:\n{}", i, eig.eigenvectors.column(i));
    // }
}
