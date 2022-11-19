//Pascal's triangle
use std::io::stdin;
pub fn run() {
    let height: usize;
    let mut input = String::new();
    let mut j: usize;
    let mut nums: Vec<u32>;
    println!("Enter height of Pascals triangle");
    stdin().read_line(&mut input).expect("could not read input");
    input.pop();
    height = input.parse().unwrap();

    nums = Vec::new();
    for i in 0..height {
        if i > 1 {
            j = i - 1;
            while j > 0 {
                nums[j] += nums[j - 1];
                j -= 1;
            }
        }

        nums.push(1);
        println!("{:?}", nums);
    }
}
