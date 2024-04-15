/*
	bfs
	This problem requires you to implement a basic BFS algorithm
*/

use std::collections::VecDeque;
use std::fmt;
use std::fmt::Formatter;

// Define a graph
struct Graph {
    adj: Vec<Vec<usize>>,
}

impl Graph {
    // Create a new graph with n vertices
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    // Add an edge to the graph
    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest);
        self.adj[dest].push(src);
    }

    // Perform a breadth-first search on the graph, return the order of visited nodes
    fn bfs_with_return(&self, start: usize) -> Vec<usize> {
        let mut visit_order = vec![];
        visit_order.push(start);
        // let mut next_nv = vec![];
        // next_nv.push(start);
        let mut si = 0;

        loop {
            let mut skip = true;
            for i in si..visit_order.len() {
                println!("visit_order => {:?}", visit_order);
                if let Some(nv) = self.adj.get(i) {
                    si += 1;
                    println!("\n{}  => {:?}\n try: ", i, nv);
                    for inv in 0..nv.len() {
                        let mut need_add = true;
                        if let Some(val) = nv.get(inv) {
                            print!("{},",*val);
                            for j in 0..visit_order.len() {
                                if let Some(value) = visit_order.get(j) {
                                    if (*value == (*val)) {
                                        need_add = false;
                                        break;
                                    }
                                }
                            }
                            if need_add {
                                visit_order.push(*val);
                                skip = false;
                            }
                        }
                    }
                }
            }
            if skip {
                break;
            }
        }

        visit_order
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_all_nodes_visited() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 4);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(1, 4);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 4, 2, 3]);
    }

    #[test]
    fn test_bfs_different_start() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visited_order = graph.bfs_with_return(2);
        assert_eq!(visited_order, vec![2, 1, 0]);
    }

    #[test]
    fn test_bfs_with_cycle() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_bfs_single_node() {
        let mut graph = Graph::new(1);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0]);
    }
}
