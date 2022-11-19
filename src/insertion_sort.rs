// insertion sort
fn sort(nums: &mut [i32]) {
    let len = nums.len();
    let mut i = 1;
    let mut j;
    let mut ele;
    while i < len {
        ele = nums[i];
        j = i - 1;
        loop {
            if nums[j] > ele {
                nums[j + 1] = nums[j];
            } else {
                nums[j + 1] = ele;
                break;
            }
            if j == 0 {
                nums[0] = ele;
                break;
            }
            j -= 1;
        }
        i += 1;
    }
}

fn main() {
    let mut nums1 = [1, 2, 6, 3, 6, 4, 7, 4, 2];
    let mut nums2 = [8, 7, 5, 4, 3, 2, 2, 1];
    let mut nums3 = [1];
    let mut nums4 = [4, 5, 6, 7, 1, 2, 3];
    sort(&mut nums1);
    sort(&mut nums2);
    sort(&mut nums3);
    sort(&mut nums4);
    println!("{:?} {:?} {:?} {:?}", nums1, nums2, nums3, nums4);
}
