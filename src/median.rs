//Median of given numbers
fn read_nums() -> Vec<f32> {
    let mut input = String::new();
    let mut str_num = String::new();
    let mut num;
    let mut nums = Vec::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("could not read input");
    for ch in input.chars() {
        if ch >= '0' && ch <= '9' || ch == '.' {
            str_num.push(ch);
        } else if str_num.len() > 0 {
            num = str_num.parse().expect("parse error, invalid input");
            nums.push(num);
            str_num.clear();
        }
    }
    nums
}
fn median(nums: &mut Vec<f32>) -> f32 {
    assert!(nums.len() > 0);
    let mut temp;
    let mut small;
    let len = nums.len();
    for start in 0..len - 1 {
        small = start;
        for i in start + 1..len {
            if nums[i] < nums[small] {
                small = i;
            }
        }
        temp = nums[start];
        nums[start] = nums[small];
        nums[small] = temp;
    }
    if len % 2 == 1 {
        nums[len / 2]
    } else {
        (nums[len / 2] + nums[len / 2 - 1]) / 2.0
    }
}

fn main() {
    let mut nums;
    let res;
    println!("Enter space separated nums");
    nums = read_nums();
    res = median(&mut nums);
    println!("Sorted nums: {nums:?}");
    println!("Median: {res}");
}
