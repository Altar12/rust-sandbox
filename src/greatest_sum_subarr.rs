//greatest sum sub array
fn sum(nums: &[i32]) -> i32 {
    let mut res: i32 = 0;
    for num in nums {
        res += num;
    }
    res
}

fn greatest_sum_subarr(nums: &[i32]) -> &[i32] {
    let mut res = &nums[..];
    let mut greatest_sum = sum(res);
    let mut sub_arr: &[i32];
    let mut temp: i32;

    for start in 0..nums.len() {
        for end in start + 1..=nums.len() {
            sub_arr = &nums[start..end];
            temp = sum(sub_arr);
            if temp > greatest_sum {
                res = sub_arr;
                greatest_sum = temp;
            }
        }
    }
    res
}

fn main() {
    let nums = [-10, 29, 45, 20, -11, 10, 22];
    let res = greatest_sum_subarr(&nums[..]);
    println!("greatest sum subarray: {:?}", res);
}
