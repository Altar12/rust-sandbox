//bubble sort
fn swap(nums: &mut [i32], index1: usize, index2: usize) {
    let temp = nums[index1];
    nums[index1] = nums[index2];
    nums[index2] = temp;
}

fn sort(nums: &mut [i32]) {
    let mut size = nums.len();
    while size > 1 {
        for index in 0..size - 1 {
            if nums[index] > nums[index + 1] {
                swap(nums, index, index + 1);
            }
        }
        size -= 1;
    }
}

#[cfg(test)]
#[test]
fn test1() {
    let mut nums = [];
    sort(&mut nums);
    assert_eq!(nums, []);
}
#[test]
fn test2() {
    let mut nums = [1, 2, 33, 22, 88, 99, 88, 34];
    sort(&mut nums);
    assert_eq!(nums, [1, 2, 22, 33, 34, 88, 88, 99]);
}

#[test]
fn test3() {
    let mut nums = [5, 4, 3, 2, 1];
    sort(&mut nums);
    assert_eq!(nums, [1, 2, 3, 4, 5]);
}
