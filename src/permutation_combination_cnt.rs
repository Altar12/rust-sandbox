//Permutation and combination calculation
fn fact(num: u32) -> u32 {
    let mut res = 1;
    for i in 2..=num {
        res *= i;
    }
    res
}
fn read(input: &mut String) {
    input.clear();
    std::io::stdin()
        .read_line(input)
        .expect("could not read input");
    input.pop();
}
fn main() {
    let n;
    let r;
    let p_cnt;
    let c_cnt;
    let input = &mut String::new();
    println!("Enter total number of objects");
    read(input);
    n = input.parse().expect("parse error, check your input");
    println!("Enter number of objects to be taken at a time");
    read(input);
    r = input.parse().expect("parse error, check your input");
    p_cnt = fact(n) / fact(n - r);
    c_cnt = p_cnt / fact(r);
    println!("Assuming all objects are unique");
    println!("nPr = {p_cnt}, nCr = {c_cnt}");
}
