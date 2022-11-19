// binary search
fn find_pos(nums: &[i32], key: i32) -> usize {
    let mut left = 0;
    let mut right = nums.len() - 1;
    let mut mid;
    let mut val;
    while left <= right {
        mid = (left + right) / 2;
        val = nums[mid];
        if val == key {
            return mid;
        } else if val < key {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    nums.len()
}

fn main() {
    println!("{}", find_pos(&[1, 3, 6, 7, 9, 11, 16, 18][..], 11));
    println!("{}", find_pos(&[1, 2, 3, 4, 5, 6, 7, 8], 22));
    println!("{}", find_pos(&[1, 4, 7, 9, 12, 14, 17], 12));
}
