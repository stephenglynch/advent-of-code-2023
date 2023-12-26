use std::fs;
use std::str;
use std::collections::{HashMap, HashSet};
use nalgebra_sparse::{CooMatrix, CsrMatrix};
use nalgebra::DMatrix;
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

fn netlist_to_vector(netlist: HashMap<String, Vec<String>>) -> (Vec<f64>, HashMap<usize, Vec<usize>>, usize) {

    // Determine total set of parts
    let mut parts = HashSet::new();
    for (comp, conn_list) in netlist.iter() {
        parts.insert(comp.clone());
        for c in conn_list {
            parts.insert(c.clone());
        }
    }

    let len = parts.len();
    let parts_map: HashMap<String, usize> = parts.iter().enumerate().map(|(i, k)| (k.clone(), i)).collect();
    let mut lap_mat = vec![0.0; len * len];
    let mut graph = HashMap::new();

    // Initialise graph
    for p in parts {
        let i = *parts_map.get(&p).unwrap();
        graph.insert(i, vec![]);
    }

    for comp1 in parts.iter() {
        let row = *parts_map.get(comp1).unwrap();
        if let Some(conn_list1) = netlist.get(comp1) {
            for c in conn_list1 {
                let col = *parts_map.get(c).unwrap();
    
                // Create symmetric adjacency matrix representing node connections
                lap_mat[len * row + col] = -1.0;
                lap_mat[len * col + row] = -1.0;
    
                // Fill diagonal with number of edges (degree matrix)
                lap_mat[len * row + row] += 1.0;

                graph.get(&row).unwrap().push(col);
            }
        }

        for (comp2, conn_list2) in netlist.iter() {
            let col = *parts_map.get(comp2).unwrap();

            if conn_list2.contains(comp1) {
                // Create symmetric adjacency matrix representing node connections
                lap_mat[len * row + col] = -1.0;
                lap_mat[len * col + row] = -1.0;

                // Fill diagonal with number of edges (degree matrix)
                lap_mat[len * row + row] += 1.0;

                graph.get(&row).unwrap().push(col);
            }
        }
    }

    (lap_mat, graph, len)
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

fn calc_cuts(graph: HashMap<usize, Vec<usize>>, group: Vec<usize>) -> usize {

}

// fn transpose(m: Vec<f64>, l: usize) -> Vec<f64> {
//     let mut n = vec![0.0; l * l];
//     for row in 0..l {
//         for col in 0..l {
//             n
//         }
//     }
// }

fn main() {
    let contents = fs::read_to_string("input/input.txt").unwrap();
    let contents = contents.trim();
    let netlist = parse_netlist(contents);
    // let (lap_mat, len) = netlist_to_vector(netlist);
    let (lap_vec, len) = netlist_to_vector(netlist);
    let lap_mat = DMatrix::from_vec(len, len, lap_vec);

    let mut coo = CooMatrix::<f64>::new(len, len);
    println!("len = {}", coo.ncols());

    coo.push_matrix(0, 0, &lap_mat);
    let csr = CsrMatrix::from(&coo);
    println!("csr = {}", csr.ncols());
    let svd = svd(&csr).unwrap();
    println!("svd.d = {}", svd.d);
    // println!("svd.s:\n{:?}", svd.s);
    println!("svd.ut:\n{}", svd.ut.t().column(13).len());

    let ut = svd.ut.t();

    println!("eigenvalues:\n{}", svd.s);

    let v = ut.column(svd.d - 1);
    let g1_len = v.iter().filter(|x| (**x) > 0.0).count();
    let g2_len = len - g1_len;
    println!("answer = {}", g1_len * g2_len);

    let v = ut.column(svd.d - 2);
    let g1_len = v.iter().filter(|x| (**x) > 0.0).count();
    let g2_len = len - g1_len;
    println!("answer = {}", g1_len * g2_len);

    let v = ut.column(svd.d - 3);
    let g1_len = v.iter().filter(|x| (**x) > 0.0).count();
    let g2_len = len - g1_len;
    println!("answer = {}", g1_len * g2_len);

    // let eig = lap_mat.symmetric_eigen();
    // println!("eigenvalues:\n{}", eig.eigenvalues);

    // for i in 0..len {
    //     println!("eigenvector {}:\n{}", i, eig.eigenvectors.column(i));
    // }
}
