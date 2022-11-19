//convert to binary representation
fn binary_rep(mut num: i32) -> String {
    let mut res = String::with_capacity(20);
    let mut remainders: Vec<u8> = Vec::new();
    if num < 0 {
        res.push('-');
    }
    loop {
        remainders.push((num % 2) as u8);
        num /= 2;
        if num == 0 {
            break;
        }
    }
    while remainders.len() > 0 {
        if remainders.pop().unwrap() == 0 {
            res.push('0');
        } else {
            res.push('1');
        }
    }
    res
}

fn main() {
    println!(
        "{}, {}, {}, {}",
        binary_rep(-10),
        binary_rep(12),
        binary_rep(15),
        binary_rep(-20)
    );
}
