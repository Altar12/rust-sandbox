/*
Implementation of Kruskals algorithm.
Kruskals algorithm is used to find minimum cost spanning tree from a graph.
Spanning tree of a graph is another graph that contains all the nodes of the original graph, but only a subset of its edges such that
 - it is possible to reach any other node from a given node
 - no cycles are present in the graph
Implementation is written for undirected graph.

graph: Vec<Vec<(usize, u32)>> represents a graph with nodes 0,1,..,graph.len()-1
if graph[1] contains an element (2, 8), then node 1 is connected to 2 with an edge of cost 8
*/
use std::collections::HashSet;
type Edge = (u32, usize, usize); // (cost, node1, node2), numerically node1<node2 to avoid duplication
pub fn min_cost_spanning_tree(graph: &Vec<Vec<(usize, u32)>>) -> Vec<Vec<(usize, u32)>> {
    if graph.len() == 0 {
        return Vec::new();
    }

    let mut remaining_edges = HashSet::<Edge>::new(); // edges not included in the graph yet
    let mut connected_pairs = HashSet::<(usize, usize)>::new();
    let mut res = Vec::new();

    // construct the set of edges
    for i in 0..graph.len() {
        res.push(Vec::new());
        for neighbour in &graph[i] {
            if i < neighbour.0 {
                remaining_edges.insert((neighbour.1, i, neighbour.0));
            }
        }
    }

    let edge_count = graph.len() - 1;
    let mut included_edge_count = 0;
    let mut edge;
    while included_edge_count < edge_count {
        edge = get_min_edge(&mut remaining_edges);
        if !connected_pairs.contains(&(edge.1, edge.2)) {
            res[edge.1].push((edge.2, edge.0));
            res[edge.2].push((edge.1, edge.0));
            update_connected(&mut connected_pairs, edge);
            included_edge_count += 1;
        }
    }
    res
}

fn get_min_edge(edges: &mut HashSet<Edge>) -> Edge {
    let mut min_cost = u32::MAX;
    let mut min_cost_edge = (0, 0, 0); // dummy initialisation
    for edge in edges.iter() {
        if edge.0 < min_cost {
            min_cost = edge.0;
            min_cost_edge = (edge.0, edge.1, edge.2);
        }
    }
    edges.remove(&min_cost_edge);
    min_cost_edge
}

fn update_connected(pairs: &mut HashSet<(usize, usize)>, new_edge: Edge) {
    let (_, node1, node2) = new_edge;
    let mut neighbours1 = vec![node1]; // all nodes connected to node1
    let mut neighbours2 = vec![node2]; // all nodes connected to node2
    for pair in pairs.iter() {
        if pair.0 == node1 {
            neighbours1.push(pair.1);
        } else if pair.1 == node1 {
            neighbours1.push(pair.0);
        } else if pair.0 == node2 {
            neighbours2.push(pair.1);
        } else if pair.1 == node2 {
            neighbours2.push(pair.0);
        }
    }
    for &node_a in neighbours1.iter() {
        for &node_b in neighbours2.iter() {
            if node_a == node_b {
                continue;
            } else if node_a < node_b {
                pairs.insert((node_a, node_b));
            } else {
                pairs.insert((node_b, node_a));
            }
        }
    }
}

pub fn get_graph_cost(graph: &Vec<Vec<(usize, u32)>>) -> u32 {
    let mut res = 0;
    for node in 0..graph.len() {
        for neighbour in &graph[node] {
            if neighbour.0 > node {
                res += neighbour.1;
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_implementation() {
        let graph = vec![
            vec![(1, 28), (5, 10)],
            vec![(0, 28), (2, 16), (6, 14)],
            vec![(1, 16), (3, 12)],
            vec![(2, 12), (4, 22), (6, 18)],
            vec![(3, 22), (5, 25), (6, 24)],
            vec![(0, 10), (4, 25)],
            vec![(1, 14), (3, 18), (4, 24)],
        ];
        let graph2 = vec![
            vec![(1, 5), (3, 4)],
            vec![(0, 5), (2, 3), (3, 2)],
            vec![(1, 3), (3, 6)],
            vec![(0, 4), (1, 2), (3, 6)],
        ];

        let spanning = min_cost_spanning_tree(&graph);
        let spanning2 = min_cost_spanning_tree(&graph2);
        assert_eq!(get_graph_cost(&spanning), 99);
        assert_eq!(get_graph_cost(&spanning2), 9);
    }
}
