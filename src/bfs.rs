use std::collections::HashSet;

//Breadth first search
fn traverse(node: u8, neighbours: &Vec<Vec<u8>>, queue: &mut Vec<u8>, inserted: &mut HashSet<u8>) {
    print!("{node}->");
    for neighbour in &neighbours[node as usize] {
        if !inserted.contains(neighbour) {
            queue.push(*neighbour);
            inserted.insert(*neighbour);
        }
    }
    if queue.len() > 0 {
        let next_node = queue.remove(0);
        traverse(next_node, neighbours, queue, inserted);
    }
}
fn traverse_graph(start_node: u8, neighbours: &Vec<Vec<u8>>) {
    print!("start->");
    traverse(start_node, neighbours, &mut Vec::new(), &mut HashSet::new());
    println!("end");
}
fn read() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("could not read input");
    input.pop();
    input
}

fn read_nums(upper_bound: u8) -> Result<Vec<u8>, ()> {
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
            num = str_num.parse::<u8>().expect("parse error, invalid input");
            if num >= upper_bound {
                return Err(());
            }
            nums.push(num);
            str_num.clear();
        }
    }
    Ok(nums)
}

fn main() {
    let mut neighbours = Vec::new();
    let node_cnt;
    let start_node;
    println!("Enter total no. of nodes");
    node_cnt = read().parse::<u8>().expect("parse error, check your input");
    for i in 0..node_cnt {
        println!("Enter neighbours of node{i}");
        match read_nums(node_cnt) {
            Ok(nums) => neighbours.push(nums),
            Err(_) => panic!("Invalid input"),
        }
    }
    println!("Enter the start node");
    start_node = read().parse::<u8>().expect("parse error, check your input");
    assert!(start_node < node_cnt, "invalid start node");
    traverse_graph(start_node, &neighbours)
}
