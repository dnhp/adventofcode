extern crate day14;
extern crate petgraph;

use petgraph::visit::{Walker,Dfs};
use petgraph::graphmap::UnGraphMap;

use day14::*;

fn main() {

    //let base_string = "ffayrhll";
    let base_string = "flqrgnkx";
    let mut nnz = 0;
    let mut bit_array: Vec<Vec<char>> = Vec::new();

    for num in 0..128 {
        let new_str = [base_string, "-", &num.to_string()[..]].concat();
        let hash = knot_hash(&new_str, 'b');
        bit_array.push(hash.chars().collect());
        nnz += hash.chars()
                .filter(|&bit| bit=='1')
                .count();
    }
    println!("Number of squares used [part1]: {:?}", nnz);
    //println!("{:?}", bit_array);

    //let mut conns: Vec<(usize,usize)> = Vec::new();

    let mut conns_h: Vec<(usize,usize)> = Vec::new();
    let mut conns_v: Vec<(usize,usize)> = Vec::new();

    let nrows = bit_array.len();
    let ncols = nrows;
    /*
    for (irow,row) in bit_array.iter().enumerate() {
        println!("irow: {:?}\nlen: {:?}", row, row.len());
        panic!();
        for icol in 0..ncols-1 {
            //println!("icol: {:?} row len: {:?}", icol, row.len());
            if (row[icol]=='1') && (row[icol]==row[icol+1]) {
                conns.push((nrows*irow + icol, nrows*irow + icol+1));
            }
        }
    }
    */

    for irow in 0..nrows {
        for icol in 0..ncols-1 {
            if (bit_array[irow][icol] == '1') && (bit_array[irow][icol] == bit_array[irow][icol+1]) {
                conns_h.push(((nrows*irow) + icol, (nrows*irow) + icol+1));
            }
        }
    }

    for icol in 0..ncols {
        for irow in 0..nrows-1 {
            if (bit_array[irow][icol] == '1') && (bit_array[irow][icol] == bit_array[irow+1][icol]) {
                conns_v.push(((nrows*irow) + icol, (nrows*(irow+1)) + icol));
            }
        }
    }

    
    
    let network = UnGraphMap::<_,()>::from_edges(&conns);
    let mut remaining_nodes: Vec<usize> = network.nodes().collect();
    println!("len nodes: {:?}", remaining_nodes.len());
    
    /*
    let mut num_groups = 0;
    
    // Step through full network starting from one node, find nodes
    // in each subnetwork, and remove them from remaining nodes list
    while !remaining_nodes.is_empty() {
        let stepper = Dfs::new(&network, *remaining_nodes.first().unwrap());
        let sub_network: Vec<usize> = stepper.iter(&network).collect();
        remaining_nodes.retain(|node| !sub_network.contains(node));
        num_groups += 1;
    }



    println!("Number of groups [part2]: {:?}", num_groups);
    */
}


//fn hash_iput