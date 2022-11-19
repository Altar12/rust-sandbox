//shifting array elements by specified positions
//negative shift value passed implies left shift, otherwise its righ shift

fn shift(nums: &mut [i32], shift: i8) {
    let len = nums.len();
    let left_shift = shift < 0;
    let mut shift = if left_shift { -shift } else { shift } as usize;
    if len < 2 || shift % len == 0 {
        return;
    }
    shift %= len;
    let (mut start1, start2) = if left_shift {
        (0usize, len - shift)
    } else {
        (len - shift, 0usize)
    };

    let mut temp = Vec::new();
    let end = start1 + shift;
    while start1 < end {
        temp.push(nums[start1]);
        start1 += 1;
    }
    if left_shift {
        for i in shift..len {
            nums[i - shift] = nums[i];
        }
    } else {
        for i in (0..len - shift).rev() {
            nums[i + shift] = nums[i];
        }
    }
    for i in 0..shift {
        nums[start2 + i] = temp[i];
    }
}

fn main() {
    let mut nums1 = [1, 2, 3, 4, 5];
    let mut nums2 = [1, 2];
    let mut nums3 = [1, 2, 3, 4, 5];
    shift(&mut nums1, 3);
    shift(&mut nums3, -3);
    shift(&mut nums2, -1);
    println!("{:?}, {:?}, {:?}", nums1, nums2, nums3);
}
