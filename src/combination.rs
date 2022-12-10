//Generating all possible combinations of a string

use std::collections::HashSet;

fn print(chars: &Vec<char>, positions: &Vec<usize>) {
    for i in 0..positions.len() {
        print!("{}", chars[positions[i]]);
    }
    println!("");
}

fn main() {
    let mut chars = Vec::new();
    let mut positions = Vec::new();
    let mut history = Vec::new();
    let len; //length of chars vector -> total number of characters
    let char_cnt; //number of characters in each combination
    let last_pos; //will be set to char_cnt - 1
    let mut input = String::new();
    let mut ptr;
    println!("Enter the string");
    std::io::stdin()
        .read_line(&mut input)
        .expect("could not read input");
    input.pop();
    for ch in input.chars() {
        chars.push(ch);
    }
    chars.sort();
    len = chars.len();
    input.clear();
    println!("Enter number of characters in each combination");
    std::io::stdin()
        .read_line(&mut input)
        .expect("could not read input");
    input.pop();
    char_cnt = input
        .parse::<usize>()
        .expect("parse error, check your input");
    if char_cnt > len || char_cnt == 0 {
        eprintln!("Invalid char count!");
        return ();
    }
    last_pos = char_cnt - 1;
    for i in 0..char_cnt {
        positions.push(i);
        history.push(HashSet::new());
        history[i].insert(chars[i]);
    }
    print(&chars, &positions);
    ptr = last_pos;
    loop {
        positions[ptr] += 1;
        if positions[ptr] == len {
            if ptr == 0 {
                break;
            }
            ptr -= 1;
            continue;
        }
        if !history[ptr].contains(&chars[positions[ptr]]) {
            history[ptr].insert(chars[positions[ptr]]);
            if ptr == last_pos {
                print(&chars, &positions);
            } else {
                ptr += 1;
                positions[ptr] = positions[ptr - 1];
                history[ptr].clear();
            }
        }
    }
}
