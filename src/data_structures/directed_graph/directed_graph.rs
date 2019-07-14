
use std::collections::{HashMap,HashSet};
use std::iter::FromIterator;
use std::u32;

pub struct DirectedGraph {
    // <source_node, Vec<(destination_node, weight)>>
    // Used HashMap for efficient lookup of a source node's neighbors.
    node_edges: HashMap<usize,Vec<(usize,u32)>>
}

impl DirectedGraph {

    pub fn new() -> Self {
        DirectedGraph {
            node_edges: HashMap::new()
        }
    }

    pub fn with_nodes(nodes: usize) -> Self {
        let mut node_edges = HashMap::with_capacity(nodes);

        for node_index in 0..nodes {
            node_edges.insert(node_index, Vec::new());
        }

        DirectedGraph {
            node_edges
        }
    }

    pub fn add_node(&mut self) -> usize {
        let node_index = self.node_edges.len();
        self.node_edges.insert(node_index, Vec::new());
        return node_index;
    }

    pub fn add_edge(&mut self, start_node: usize, end_node: usize, weight: u32) {
        match &mut self.node_edges.get_mut(&start_node) {
            Some(edge_list) => edge_list.push((end_node, weight)),
            None => eprintln!("Failed to create edge: start node {} doesn't exist in this graph!", &start_node)
        };
    }

    pub fn well_formed(&self) -> bool {
        self.node_edges.values()
            .all(|edge_list| edge_list.iter()
                .all(|edge| self.node_edges.contains_key(&edge.0)))
    }

    // Dijkstra's algorithm
    pub fn distance(&self, start_node: usize, end_node: usize) -> u32 {

        let mut distance: Vec<u32> = Vec::with_capacity(self.node_edges.len());
        for n in 0..self.node_edges.len() {
            if n == start_node {
                distance.insert(n, 0);
            } else {
                distance.insert(n, u32::MAX);
            }
        }

        let mut unvisited: HashSet<usize> = HashSet::from_iter(0..self.node_edges.len());
        let mut current_node = start_node;

        loop {
            for edge in self.node_edges.get(&current_node).unwrap() {
                let neighbor_node = edge.0;
                let current_distance = distance[neighbor_node];
        
                let weight = edge.1;
                let new_distance = distance[current_node] + weight;

                if new_distance < current_distance {
                    distance[neighbor_node] = new_distance;
                }
            }

            unvisited.remove(&current_node);

            println!("{}", unvisited.len());
            let next_node = unvisited.iter().min_by(|&n1, &n2| distance[*n1].cmp(&distance[*n2]));
            if !unvisited.contains(&end_node) || match next_node { None => true, Some(n) => distance[*n] == u32::MAX } {
                break;
            } else {
                current_node = *next_node.unwrap();
            }
        }

        return distance[end_node];
    }
}