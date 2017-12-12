extern crate petgraph;
extern crate regex;

use std::fs::File;
use std::io::{BufRead,BufReader};

use petgraph::visit::{Walker,Dfs};
use petgraph::graphmap::UnGraphMap;

use regex::Regex;

fn main() {

    let f = BufReader::new(File::open("input.txt").unwrap());
    let data: Vec<String> = f.lines()
                            .map(|l| l.unwrap())
                            .collect();

    let re = Regex::new(r"(\s<->\s)").unwrap();

    let mut all_edges: Vec<(usize, usize)> = Vec::new();

    // Iterate over input data, collecting each connection
    // to be made into a vector
    for line in &data {
        let after = re.replace_all(line, ", ");

        let node_conns: Vec<usize> = after.split(", ")
                                .map(|c| c.parse().unwrap())
                                .collect();

        for conn in node_conns[1..].iter() {
            all_edges.push((node_conns[0], *conn));
        }
    }

    // Construct an undirected graph fron the vector of
    // edges we have constructed
    let network = UnGraphMap::<_,()>::from_edges(&all_edges);

    // Step through the sub-network from node 0 with depth-first
    // search and count how many nodes are in the sub-network
    let stepper = Dfs::new(&network, 0);
    let n0: Vec<usize> = stepper.iter(&network).collect();

    println!("Nodes connected to node 0 [part1]: {:?}", n0.len());

    let mut remaining_nodes: Vec<usize> = network.nodes().collect();
    let mut num_groups = 0;

    // Step through full network starting from one node, find nodes
    // in each subnetwork, and remove them from remaining nodes list
    while !remaining_nodes.is_empty() {
        let stepper = Dfs::new(&network, *remaining_nodes.first().unwrap());
        let sub_network: Vec<usize> = stepper.iter(&network).collect();
        get_nodes_remaining(&mut remaining_nodes, &sub_network);
        num_groups += 1;
    }

    println!("Total number of sub-networks [part2]: {:?}", num_groups);
}

fn get_nodes_remaining (
    remaining_nodes: & mut Vec<usize>,
    nodes_to_remove: &Vec<usize>) {

    remaining_nodes.retain(|node| !nodes_to_remove.contains(node));
}