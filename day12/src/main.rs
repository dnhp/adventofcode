extern crate petgraph;
extern crate regex;

use petgraph::{Graph};
use petgraph::graph::node_index;
use petgraph::prelude::*;
use petgraph::visit::{Walker,Dfs};
use petgraph::EdgeDirection::{Outgoing, Incoming};
use std::fs::File;
use std::io::{BufRead,BufReader};
use regex::Regex;
use std::collections::HashMap;

fn main() {

    let f = BufReader::new(File::open("test.txt").unwrap());
    let data: Vec<String> = f.lines()
                            .map(|l| l.unwrap())
                            .collect();

    let re_pipe = Regex::new(r"(\s<->\s)").unwrap();

    let mut network: Graph<usize, usize, _> = Graph::new_undirected();

    let mut map = HashMap::new();



    // Iterate over list, add required no. of nodes to graph,
    // collect connections to be made
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

    let mut remaining_nodes: Vec<usize> = (0..network.node_count()).collect();
    //println!("rn {:?}", remaining_nodes);
    // Create connections in graph by reading data stored in
    // hashmap
    for (_, conn_vec) in map.iter() {
        for conn in conn_vec {
            if conn.0 != conn.1 {
                network.update_edge(node_index(conn.0), node_index(conn.1), 1);
            }
        }
    }
    
    // Step through all nodes connected to node 0 until return none
    let stepper = Dfs::new(&network, node_index(0));
    let nodes_connected_to_0: Vec<NodeIndex> = stepper.iter(&network).collect();

    println!("nodes connected to node 0 [part1]: {:?}", nodes_connected_to_0.len());

    let mut num_groups = 0;

    let singletons: Vec<NodeIndex> = network.externals(Outgoing).collect();

    num_groups += singletons.len();

    println!("No. of singletons: {:?}", singletons.len());
    
    for single_node in singletons {
        network.remove_node(single_node);    
    }


    for node in nodes_connected_to_0 {
        network.remove_node(node);
    }

    num_groups += 1;

    while network.node_count() > 0 {

    }
    

    //let nodes_in_g0 = 


    //
    //let r = network.without_edges(Outgoing).count();
    //println!("r{:?}", r);

}
