/*
Implementation of Prim's algorithm.
Prims algorithm is used to find minimum cost spanning tree from a graph.
Spanning tree of a graph is another graph that contains all the nodes of the original graph, but only a subset of its edges such that
 - it is possible to reach any other node from a given node
 - no cycles are present in the graph
Implementation is written for undirected graph.

graph: Vec<Vec<(usize, u32)>> represents a graph with nodes 0,1,..,graph.len()-1
if graph[1] contains an element (2, 8), then node 1 is connected to 2 with an edge of cost 8
*/
use std::collections::HashSet;

pub fn minimum_spanning_tree(graph: &Vec<Vec<(usize, u32)>>) -> Vec<Vec<(usize, u32)>> {
    if graph.len() == 0 {
        return Vec::new();
    }

    let mut included = HashSet::new();
    let mut res = Vec::new();
    let mut min_cost;
    let node_count = graph.len();
    for _ in 0..graph.len() {
        res.push(Vec::new())
    }
    // select the minimum cost edge
    {
        min_cost = (0, 0, u32::MAX);
        let mut curr_node = 0;
        for neighbour_set in graph {
            for neighbour in neighbour_set {
                if neighbour.1 < min_cost.2 {
                    min_cost = (curr_node, neighbour.0, neighbour.1);
                }
            }
            curr_node += 1;
        }
        included.insert(min_cost.0);
        included.insert(min_cost.1);
        res[min_cost.0].push((min_cost.1, min_cost.2));
        res[min_cost.1].push((min_cost.0, min_cost.2));
    }

    // recursively select the minimum cost edge that connects from an included node to non-included node
    while included.len() < node_count {
        min_cost.2 = u32::MAX;
        for node in 0..graph.len() {
            if !included.contains(&node) {
                continue;
            }
            for neighbour in &graph[node] {
                if included.contains(&neighbour.0) {
                    continue;
                }
                if neighbour.1 < min_cost.2 {
                    min_cost = (node, neighbour.0, neighbour.1);
                }
            }
        }
        res[min_cost.0].push((min_cost.1, min_cost.2));
        res[min_cost.1].push((min_cost.0, min_cost.2));
        included.insert(min_cost.1);
    }

    res
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

        let spanning = minimum_spanning_tree(&graph);
        let spanning2 = minimum_spanning_tree(&graph2);
        assert_eq!(get_graph_cost(&spanning), 99);
        assert_eq!(get_graph_cost(&spanning2), 9);
    }
}
