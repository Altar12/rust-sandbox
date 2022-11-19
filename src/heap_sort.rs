fn heapify(nums: &mut [i32], len: usize) {
    if len <= 1 {
        return;
    }
    let mut parent = len / 2 - 1;
    let mut child;
    let mut temp;
    let mut modified;
    loop {
        modified = false;
        loop {
            child = parent * 2;
            if child + 1 < len && nums[child + 1] > nums[child] {
                child += 1;
            }
            if nums[parent] < nums[child] {
                temp = nums[parent];
                nums[parent] = nums[child];
                nums[child] = temp;
                modified = true;
            }
            if parent == 0 {
                break;
            }
            parent -= 1;
        }
        if !modified {
            break;
        }
    }
}

fn sort(nums: &mut [i32]) {
    let mut size = nums.len();
    let mut temp;
    while size > 1 {
        heapify(nums, size);
        temp = nums[size - 1];
        nums[size - 1] = nums[0];
        nums[0] = temp;
        size -= 1;
    }
}

fn main() {
    let mut nums1 = [7, 6, 5, 4, 3, 1, 3];
    let mut nums2 = [5, 6, 7, 8, 4, 3, 2, 1];
    let mut nums3 = [1, 2, 4, 3, 2, 1, 5, 6, 3];
    sort(&mut nums1[..]);
    sort(&mut nums2[..]);
    sort(&mut nums3[..]);
    println!("{:?}", nums1);
    println!("{:?}", nums2);
    println!("{:?}", nums3);
}
