fn run() {
    let mut nums: Vec<i32> = Vec::new();
    let mut input = String::new();
    let mut res: i32 = 0;
    let mut len: usize = 0;
    println!("Enter postfix expression");
    std::io::stdin()
        .read_line(&mut input)
        .expect("could not read input");
    input.pop();
    for ch in input.chars() {
        if ch == '+' {
            nums[len - 1] += res;
            res = 0;
        } else if ch == '-' {
            nums[len - 1] -= res;
            res = 0;
        } else if ch == '*' {
            nums[len - 1] *= res;
            res = 0;
        } else if ch == '/' {
            nums[len - 1] /= res;
            res = 0;
        } else if ch == ',' {
            nums.push(res);
            res = 0;
            len += 1;
        } else {
            res = res * 10 + (ch as i32) - 48;
        }
    }
    println!("Result: {}", nums[0]);
}
