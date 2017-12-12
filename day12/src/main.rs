extern crate petgraph;
extern crate regex;

use std::fs::File;
use std::io::{BufRead,BufReader};
use std::collections::HashMap;

use petgraph::graph::node_index;
use petgraph::prelude::*;
use petgraph::visit::{Walker,Dfs};
use petgraph::EdgeDirection::Outgoing;

use regex::Regex;


fn main() {

    let f = BufReader::new(File::open("input.txt").unwrap());
    let data: Vec<String> = f.lines()
                            .map(|l| l.unwrap())
                            .collect();

    let re_pipe = Regex::new(r"(\s<->\s)").unwrap();

    let mut network: Graph<usize, usize, _> = Graph::new_undirected();

    let mut map = HashMap::new();

    // Iterate over input data, add required no. of nodes to graph,
    // collect connections to be made in tuples and store in
    // hashmap for later retrieval
    for line in &data {
        let after = re_pipe.replace_all(line, ", ");

        let conn: Vec<usize> = after.split(", ")
                                .map(|c| c.parse().unwrap())
                                .collect();

        let mut conn_node_vec: Vec<(usize, usize)> = Vec::new();

        for conn_node in conn[1..].iter() {
            let conn_tuple = (conn[0], *conn_node);
            conn_node_vec.push(conn_tuple);
        }
        network.add_node(0).index();
        map.insert(conn[0], conn_node_vec);
    }

    // Create connections in graph by iterating over all data
    // stored in hashmap and applying the connection tuple for
    // each node (if it is not a singleton)
    for (_, conn_vec) in map.iter() {
        for conn in conn_vec {
            if conn.0 != conn.1 {
                network.update_edge(node_index(conn.0), node_index(conn.1), 1);
            }
        }
    }
    
    // Step through all nodes connected to node 0 until none left
    let stepper = Dfs::new(&network, node_index(0));
    let nodes_connected_to_0: Vec<NodeIndex> = stepper.iter(&network).collect();

    println!("nodes connected to node 0 [part1]: {:?}", nodes_connected_to_0.len());

    /* Part 2 */
    let mut num_groups = 0; // counter for number of separate sub-networks

    // Build a vector containing all the nodes in the network
    let mut remaining_nodes: Vec<NodeIndex> = (0..network.node_count())
                                                .map(|x| node_index(x))
                                                .collect();

    // Remove the nodes we have already accounted for, leaving in the
    // network those nodes that we haven't stepped into yet
    get_nodes_remaining(&mut remaining_nodes, &nodes_connected_to_0);

    // Increment num_groups counter as we have removed the network from part 1
    num_groups += 1;

    // Find and remove nodes which have no connections to any other nodes.
    // Each of these nodes counts as a group
    let singletons: Vec<NodeIndex> = network.externals(Outgoing).collect();
    get_nodes_remaining(&mut remaining_nodes, &singletons);
    num_groups += singletons.len();

    // Loop over list of remaining nodes, finding full sub-networks for
    // each and removing these sub-networks from the remaining node list.
    while !remaining_nodes.is_empty() {
        let stepper = Dfs::new(&network, *remaining_nodes.first().unwrap());
        let sub_network: Vec<NodeIndex> = stepper.iter(&network).collect();
        get_nodes_remaining(&mut remaining_nodes, &sub_network);
        num_groups += 1;
    }

    println!("Total number of groups [part2]: {:?}", num_groups);
}

fn get_nodes_remaining (
    remaining_nodes: & mut Vec<NodeIndex>,
    nodes_to_remove: &Vec<NodeIndex>) {

    remaining_nodes.retain(|node| !nodes_to_remove.contains(node));
}