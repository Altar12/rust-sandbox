//merge sort

fn sort(nums: &mut [i32], start: usize, len: usize) {
    if len <= 1 {
        return;
    }
    sort(nums, start, len / 2);
    sort(nums, start + len / 2, len - len / 2);
    combine(nums, start, len);
}

fn combine(nums: &mut [i32], start: usize, len: usize) {
    let end1 = start + len / 2;
    let end2 = start + len;
    let vec_len = len / 2;
    let mut nums1 = Vec::with_capacity(len / 2);
    let mut i = 0;
    let mut j = start;
    let mut k = start;
    while j < end1 {
        nums1.push(nums[j]);
        j += 1;
    }
    while i < vec_len && j < end2 {
        if nums1[i] < nums[j] {
            nums[k] = nums1[i];
            i += 1;
        } else {
            nums[k] = nums[j];
            j += 1;
        }
        k += 1;
    }
    if j == end2 {
        while k < end2 {
            nums[k] = nums1[i];
            k += 1;
            i += 1;
        }
    }
}

#[cfg(test)]
#[test]
fn test1() {
    let mut nums = [1, 5, 7, 2, 4, 7, 9];
    let len = nums.len();
    sort(&mut nums[..], 0, len);
    assert_eq!(nums, [1, 2, 4, 5, 7, 7, 9]);
}

#[test]
fn test2() {
    let mut nums = [9, 8, 7, 6, 5, 4, 4, 3];
    let len = nums.len();
    sort(&mut nums[..], 0, len);
    assert_eq!(nums, [3, 4, 4, 5, 6, 7, 8, 9]);
}
