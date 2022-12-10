//Generating all possible permutations of a string
use std::collections::HashSet;
use std::io::stdin;

fn print(chars: &Vec<char>, positions: &Vec<usize>) {
    for i in 0..positions.len() {
        print!("{}", chars[positions[i]]);
    }
    println!("");
}

fn main() {
    let mut chars = Vec::new();
    let mut positions = Vec::new();
    let mut histories = Vec::new();
    let mut occupied = HashSet::new();
    let len;
    let char_cnt;
    let last_ptr;
    let mut ptr;
    let mut pos;
    let mut input = String::new();
    println!("Enter the string");
    stdin().read_line(&mut input).expect("could not read input");
    input.pop();
    for ch in input.chars() {
        chars.push(ch);
    }
    len = input.len();
    input.clear();
    println!("Enter number of characters in a permutation");
    stdin().read_line(&mut input).expect("could not read input");
    input.pop();
    char_cnt = input.parse().expect("parse error, check your input");
    if char_cnt == 0 || char_cnt > len {
        eprintln!("Invalid char count!");
        return ();
    }
    last_ptr = char_cnt - 1;
    for i in 0..char_cnt {
        positions.push(i);
        histories.push(HashSet::new());
        histories[i].insert(chars[i]);
        occupied.insert(i);
    }
    occupied.remove(&last_ptr);
    histories[last_ptr].clear();
    ptr = last_ptr;
    loop {
        pos = positions[ptr];
        if pos == len {
            if ptr == 0 {
                break;
            }
            ptr -= 1;
            occupied.remove(&positions[ptr]);
            continue;
        }
        if !occupied.contains(&pos) && !histories[ptr].contains(&chars[pos]) {
            if ptr == last_ptr {
                print(&chars, &positions);
                histories[ptr].insert(chars[pos]);
            } else {
                occupied.insert(pos);
                histories[ptr].insert(chars[pos]);
                ptr += 1;
                histories[ptr].clear();
                positions[ptr] = 0;
                continue;
            }
        }
        positions[ptr] += 1;
    }
}
