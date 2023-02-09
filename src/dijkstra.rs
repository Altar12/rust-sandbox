// Implementation of dijkstra's algorithm
// Used to find the lengths of shortest paths in a graph from a starting vertex to all the vertices

/*
Number of nodes in the graph = number of elements in the 'graph' vector
Nodes are represented by numbers
For eg if graph.len() == 5, then 5 nodes are present, namely 0,1,2,3,4
The information about neighbours of a node 'x' are present in the vector graph[x]
For example if graph[2] contains the element (5,8), then 2 is connected to 5 with an edge of length 8
The function returns a vector containing vertices and their shortest distance from start vertex
i.e. if start_vertex = 0 is passed and the return value of function contains (3, 9) that means shortest path from 0 to 3 is of  length 9
however if (3, u32::MAX) is retuned then there is no path from 0 to 3
*/
pub fn shortest_paths(graph: &Vec<Vec<(usize, u32)>>, start_vertex: usize) -> Vec<(usize, u32)> {
    let mut res = Vec::new();
    let mut remaining: Vec<(usize, u32)>;
    let mut nearest: usize; // nearest vertex to start_vertex in remaining vector
    let mut nearest_index: usize; // index of the nearest vertex in remaining vector
    for i in 0..graph.len() {
        res.push((i, u32::MAX));
    }
    for neighbour in graph[start_vertex].iter() {
        res[neighbour.0].1 = neighbour.1;
    }
    res[start_vertex].1 = 0;
    remaining = res.clone();
    while remaining.len() > 0 {
        nearest_index = least_distance_index(&remaining).unwrap();
        if remaining[nearest_index].1 == u32::MAX {
            break; // all remaining vertices are unreachable
        }
        nearest = remaining[nearest_index].0;
        for neighbour in graph[nearest].iter() {
            if res[neighbour.0].1 > res[nearest].1 + neighbour.1 {
                res[neighbour.0].1 = res[nearest].1 + neighbour.1;
            }
        }
        remaining.remove(nearest_index);
        for i in 0..remaining.len() {
            remaining[i].1 = res[remaining[i].0].1;
        }
    }
    return res;
}

fn least_distance_index(vertices: &Vec<(usize, u32)>) -> Option<usize> {
    if vertices.len() == 0 {
        return None;
    }
    let mut nearest = 0;
    for i in 1..vertices.len() {
        if vertices[i].1 < vertices[nearest].1 {
            nearest = i;
        }
    }
    Some(nearest)
}

#[cfg(test)]
mod test {
    use super::shortest_paths;
    #[test]
    fn test_dijkstra() {
        let input_graph = vec![
            vec![(1, 2), (2, 4)],
            vec![(2, 1), (3, 7)],
            vec![(4, 3)],
            vec![(5, 1)],
            vec![(3, 2), (5, 5)],
            vec![],
        ];
        let input_graph_second = vec![
            vec![(1, 50), (2, 45), (3, 10)],
            vec![(2, 10), (3, 15)],
            vec![(4, 30)],
            vec![(0, 10), (4, 15)],
            vec![(1, 20), (2, 35)],
            vec![],
        ];

        let mut distances = shortest_paths(&input_graph, 0);
        assert_eq!(
            distances,
            vec![(0, 0), (1, 2), (2, 3), (3, 8), (4, 6), (5, 9)]
        );
        distances = shortest_paths(&input_graph_second, 0);
        assert_eq!(
            distances,
            vec![(0, 0), (1, 45), (2, 45), (3, 10), (4, 25), (5, u32::MAX)]
        );
    }
}
