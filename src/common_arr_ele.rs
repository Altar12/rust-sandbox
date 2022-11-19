fn intersection<'a>(nums1: &'a [i32], nums2: &'a [i32]) -> &'a [i32] {
    let (small, big) = if nums1.len() < nums2.len() {
        (nums1, nums2)
    } else {
        (nums2, nums1)
    };
    let (len1, len2) = (small.len(), big.len());
    let mut len = len1;
    while len > 0 {
        for start1 in 0..=len1 - len {
            for start2 in 0..=len2 - len {
                if &small[start1..start1 + len] == &big[start2..start2 + len] {
                    return &small[start1..start1 + len];
                }
            }
        }
        len -= 1;
    }
    &[][..]
}

fn main() {
    println!(
        "{:?}",
        intersection(&[1, 2, 3, 4, 5, 6, 7][..], &[11, 23, 56, 3, 4, 56, 76][..])
    );
    println!(
        "{:?}",
        intersection(&[1, 2, 3, 4, 5][..], &[5, 4, 2, 3, 1][..])
    );
    println!("{:?}", intersection(&[1, 2, 3, 4, 5][..], &[9, 8, 8][..]));
}
