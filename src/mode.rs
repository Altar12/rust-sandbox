use std::collections::HashMap;

fn read_nums() -> Vec<i32> {
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

fn mode(nums: &Vec<i32>) -> Vec<i32> {
    assert!(nums.len() > 0);
    let mut frequencies = HashMap::new();
    let mut res = Vec::new();
    let mut highest = 0u32;
    for num in nums {
        if frequencies.contains_key(num) {
            frequencies.insert(*num, frequencies.get(num).unwrap() + 1);
        } else {
            frequencies.insert(*num, 1);
        }
    }
    for (num, freq) in frequencies {
        if freq > highest {
            res.clear();
            res.push(num);
            highest = freq;
        } else if freq == highest {
            res.push(num);
        }
    }
    res
}

fn main() {
    let nums;
    let modes;
    println!("Enter space separated nums");
    nums = read_nums();
    modes = mode(&nums);
    println!("Nums with highest frequency: {modes:?}");
}
