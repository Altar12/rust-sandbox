// Average of n numbers

fn main() {
    let nums = [10, 29, 55, 23, 54, 54];
    let mut sum = 0;
    let length = nums.len();
    let mut i = 0usize;
    let avg: f32;

    while i < length {
        sum += nums[i];
        i += 1;
    }
    avg = (sum as f32) / (length as f32);
    println!("Nos: {:?}", nums);
    println!("Avg of nos: {avg}");
}
