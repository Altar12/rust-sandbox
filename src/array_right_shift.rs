fn shift(nums: &mut [i32], mut shift: usize) {
    let mut temp: Vec<i32> = Vec::new();
    let len = nums.len();
    let mut i;

    if len == 0 || shift % len == 0 {
        return;
    } else {
        shift %= len;
    }
    for &num in &nums[len - shift..] {
        temp.push(num);
    }
    i = len - 1;
    while i >= shift {
        nums[i] = nums[i - shift];
        i -= 1;
    }
    loop {
        nums[i] = temp.pop().unwrap();
        if i == 0 {
            break;
        }
        i -= 1;
    }
}

fn main() {
    let mut nums1 = [1, 2, 3, 4, 5];
    let mut nums2 = [1, 2];
    let mut nums3 = [23];
    let mut nums4 = [5; 0];
    shift(&mut nums1, 3);
    shift(&mut nums2, 3);
    shift(&mut nums3, 2);
    shift(&mut nums4, 4);
    println!("{:?}, {:?}, {:?}, {:?}", nums1, nums2, nums3, nums4);
}
