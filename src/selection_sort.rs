//selection sort
fn swap(nums: &mut [i32], index1: usize, index2: usize) {
    let temp = nums[index1];
    nums[index1] = nums[index2];
    nums[index2] = temp;
}

fn sort(nums: &mut [i32]) {
    let len = nums.len();
    let mut size = len;
    let mut smallest;
    while size > 1 {
        smallest = len - size;
        for i in smallest + 1..len {
            if nums[i] < nums[smallest] {
                smallest = i;
            }
        }
        swap(nums, len - size, smallest);
        size -= 1;
    }
}

#[cfg(test)]
#[test]
fn test1() {
    let mut nums = [1, 4, 2, 6, 3, 2, 6, 5];
    sort(&mut nums[..]);
    println!("{:?}", nums);
    assert_eq!(nums, [1, 2, 2, 3, 4, 5, 6, 6]);
}
#[test]
fn test2() {
    let mut nums = [9, 8, 7, 6, 5, 4, 3, 2, 1];
    sort(&mut nums[..]);
    println!("{:?}", nums);
    assert_eq!(nums, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
}
#[test]
fn test3() {
    let mut nums = [23];
    sort(&mut nums[..]);
    println!("{:?}", nums);
    assert_eq!(nums, [23]);
}
