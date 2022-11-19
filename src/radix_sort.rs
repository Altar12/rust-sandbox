// Radix sort
fn digit_cnt(mut num: i32) -> u32 {
    if num == 0 {
        return 1;
    }
    let mut res = 0;
    while num != 0 {
        num /= 10;
        res += 1;
    }
    res
}

fn find_largest(nums: &[i32]) -> Result<i32, ()> {
    if nums.len() == 0 {
        return Err(());
    }
    let mut big = std::i32::MIN;
    for &num in nums {
        if num > big {
            big = num;
        }
    }
    Ok(big)
}

fn sort(nums: &mut [i32]) {
    let big = find_largest(nums).unwrap();
    let mut cnt = digit_cnt(big);
    let mut bins: Vec<Vec<i32>> = Vec::new();
    let mut div = 1;
    let mut j;
    for _ in 0..10 {
        bins.push(Vec::new());
    }
    while cnt > 0 {
        j = 0;
        for &num in &nums[..] {
            bins[(num / div) as usize % 10].push(num);
        }
        for i in 0..10 {
            while bins[i].len() > 0 {
                nums[j] = bins[i].remove(0);
                j += 1;
            }
        }
        div *= 10;
        cnt -= 1;
    }
}

fn main() {
    let mut nums1 = [1, 2, 3, 4, 9, 8, 6, 7, 5, 4];
    let mut nums2 = [8, 7, 6, 5, 4, 3, 2, 1];
    let mut nums3 = [9, 8, 7, 6, 5, 1, 4, 2, 3];
    sort(&mut nums1);
    sort(&mut nums2);
    sort(&mut nums3);
    println!("{:?}\n{:?}\n{:?}", nums1, nums2, nums3);
}
