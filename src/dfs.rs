use std::collections::HashSet;

//Depth first search
fn traverse(node: u8, neighbours: &Vec<Vec<u8>>, traversed: &mut HashSet<u8>) {
    print!("{node}->");
    traversed.insert(node);
    for neighbour in &neighbours[node as usize] {
        if !traversed.contains(neighbour) {
            traverse(*neighbour, neighbours, traversed);
        }
    }
}
fn traverse_graph(start_node: u8, neighbours: &Vec<Vec<u8>>) {
    let mut traversed = HashSet::new();
    print!("start->");
    traverse(start_node, neighbours, &mut traversed);
    println!("end");
}
fn read_nums() -> Vec<u8> {
    let mut input = String::new();
    let mut str_num = String::new();
    let mut num;
    let mut nums = Vec::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("could not read input");
    for ch in input.chars() {
        if ch >= '0' && ch <= '9' {
            str_num.push(ch);
        } else if str_num.len() > 0 {
            num = str_num.parse().expect("parse error, invalid input");
            nums.push(num);
            str_num.clear();
        }
    }
    nums
}
fn read() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("could not read input");
    input.pop();
    input
}
fn main() {
    let mut neighbours = Vec::new();
    let start_node;
    let node_cnt;
    println!("Enter no. of nodes in graph");
    node_cnt = read()
        .parse::<usize>()
        .expect("parse error, check your input");
    for i in 0..node_cnt {
        println!("Enter neighbours of node{i}:");
        neighbours.push(read_nums());
    }
    println!("Enter starting node");
    start_node = read().parse::<u8>().expect("parse error, check your input");
    println!("DFS traversal:");
    traverse_graph(start_node, &neighbours);
}
