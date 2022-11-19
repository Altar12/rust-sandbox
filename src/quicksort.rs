//quicksort
fn partition(nums: &mut [i32], start: usize, end: usize) -> usize {
    let pivot = nums[start];
    let mut i = start + 1;
    let mut j = end;
    let mut temp;
    loop {
        while i < end && nums[i] < pivot {
            i += 1;
        }
        while j > start && nums[j] >= pivot {
            j -= 1;
        }
        if i < j {
            temp = nums[i];
            nums[i] = nums[j];
            nums[j] = temp;
        } else {
            nums[start] = nums[j];
            nums[j] = pivot;
            return j;
        }
    }
}

fn sort(nums: &mut [i32], start: usize, end: usize) {
    if end <= start || end >= nums.len() {
        return;
    }
    let pivot_pos = partition(nums, start, end);
    if pivot_pos > 0 {
        sort(nums, start, pivot_pos - 1);
    }
    sort(nums, pivot_pos + 1, end);
}

fn main() {
    let mut nums1 = [1, 6, 3, 6, 2, 6, 8, 9, 3, 2];
    let mut nums2 = [8, 7, 6, 5, 4, 3, 2];
    let mut nums3 = [6, 7, 8, 9, 1, 2, 3, 4, 5];
    sort(&mut nums1[..], 0, 9);
    sort(&mut nums2[..], 0, 6);
    sort(&mut nums3[..], 0, 8);
    println!("{:?}", nums1);
    println!("{:?}", nums2);
    println!("{:?}", nums3);
}
