//Average of n numbers
fn read(input: &mut String) {
    input.clear();
    std::io::stdin()
        .read_line(input)
        .expect("could not read input");
    input.pop();
}

fn main() {
    let mut nums: Vec<i32>;
    let mut num: i32;
    let cnt: usize;
    let mut i: usize;
    let mut input = String::with_capacity(8);
    let mut sum: i32;
    let avg: f32;
    println!("How many nos to enter:");
    read(&mut input);
    cnt = input.parse().expect("invalid input");
    nums = Vec::with_capacity(cnt);
    println!("Enter {cnt} nos");
    i = 0;
    while i < cnt {
        read(&mut input);
        num = input.parse().expect("invalid input");
        nums.push(num);
        i += 1;
    }

    i = 0;
    sum = 0;
    while i < cnt {
        sum += nums[i];
        i += 1;
    }
    avg = (sum as f32) / (cnt as f32);
    println!("Avg: {avg}");
}
