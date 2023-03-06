/*
Detecting cycle in an undirected graph with disjoint sets.
The function below expects an undirected graph. It returns true if the graph contains a cycle, false otherwise.
 */
use std::collections::HashSet;
// graph interpretation: if graph[0] = (3, 10), then node 0 is connected to node 3 with an edge of weight 10
pub fn contains_cycle(graph: &Vec<Vec<(usize, u32)>>) -> bool {
    let mut sets = Vec::new();
    for i in 0..graph.len() {
        // creating a set for each element
        sets.push(HashSet::new());
        sets[i].insert(i);
    }
    // traverse all the edges to check for cycle
    let mut set_index1;
    let mut set_index2;
    let mut temp_set;
    for i in 0..graph.len() {
        for &neighbour in &graph[i] {
            if i < neighbour.0 {
                // to avoid edge repetition
                (set_index1, set_index2) = find(&sets, (i, neighbour.0));
                if set_index1 == set_index2 {
                    return true;
                }
                if set_index1 > set_index2 {
                    // ensuring index1 < index2
                    (set_index1, set_index2) = (set_index2, set_index1);
                }
                temp_set = sets.remove(set_index2);
                sets[set_index1].extend(temp_set.into_iter());
            }
        }
    }
    false
}

// the function accepts a vector of hashsets, and a pair of elements
// and returns the indices of hashsets to which the elements belong
fn find(sets: &Vec<HashSet<usize>>, pair: (usize, usize)) -> (usize, usize) {
    let mut res = (0, 0);
    let mut found_one = false;
    let mut to_find = pair.0;
    for i in 0..sets.len() {
        if sets[i].contains(&to_find) {
            if found_one {
                res.1 = i;
                return res;
            } else {
                res.0 = i;
                found_one = true;
                to_find = pair.1;
            }
        }
    }
    res
}

mod test {
    use super::*;
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
        let graph3 = vec![
            vec![(1, 1), (2, 1), (3, 1)],
            vec![(0, 1), (4, 1)],
            vec![(0, 1), (5, 1), (6, 1)],
            vec![(0, 1)],
            vec![(1, 1)],
            vec![(2, 1)],
            vec![(2, 1)],
        ];
        assert!(contains_cycle(&graph));
        assert!(contains_cycle(&graph2));
        assert!(!contains_cycle(&graph3));
    }
}
