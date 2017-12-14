extern crate day14;
extern crate petgraph;

use petgraph::visit::{Walker,Dfs};
use petgraph::graphmap::UnGraphMap;

use day14::*;

fn main() {

    let base_string = "ffayrhll";
    let mut num_nonzero = 0;
    let mut bit_array: Vec<Vec<char>> = Vec::new();

    for num in 0..128 {
        let new_str = [base_string, "-", &num.to_string()[..]].concat();
        let hash = knot_hash(&new_str, 'b');
        bit_array.push(hash.chars().collect());
        num_nonzero += hash.chars()
                        .filter(|&bit| bit=='1')
                        .count();
    }
    println!("Number of squares used [part1]: {:?}", num_nonzero);

    let mut conns: Vec<(usize,usize)> = Vec::new();

    let nrows = bit_array.len();
    let ncols = nrows;

    // Horizontal sweep over grid, looking for connections
    for irow in 0..nrows {
        for icol in 0..ncols-1 {
            if (bit_array[irow][icol] == '1') && (bit_array[irow][icol] == bit_array[irow][icol+1]) {
                conns.push(((nrows*irow) + icol, (nrows*irow) + icol+1));
            }
        }
    }

    // Vertical sweep over grid, looking for connections
    for icol in 0..ncols {
        for irow in 0..nrows-1 {
            if (bit_array[irow][icol] == '1') && (bit_array[irow][icol] == bit_array[irow+1][icol]) {
                conns.push(((nrows*irow) + icol, (nrows*(irow+1)) + icol));
            }
        }
    }

    // Build graph from detected connections, not including singletons
    let network = UnGraphMap::<_,()>::from_edges(&conns);
    let mut remaining_nodes: Vec<usize> = network.nodes().collect();
    
    let mut num_groups = 0;
    let mut num_nodes_in_groups = 0;
    
    // Step through full network starting from one node, find nodes
    // in each subnetwork, and remove them from remaining nodes list
    while !remaining_nodes.is_empty() {
        let stepper = Dfs::new(&network, *remaining_nodes.first().unwrap());
        let sub_network: Vec<usize> = stepper.iter(&network).collect();
        remaining_nodes.retain(|node| !sub_network.contains(node));
        num_groups += 1;
        num_nodes_in_groups += sub_network.len();
    }

    // Now add the singletons, not picked up by previous loops. Number of singletons
    // equal to number of '1' entries in matrix minus the number ofnodes in groups
    let num_singletons = num_nonzero - num_nodes_in_groups;
    num_groups += num_singletons;

    println!("Number of groups [part2]: {:?}", num_groups);
}