//Partitioning a slice into given number of slices

//Divide nums slice into n number of slices
fn partition<'a>(nums: &'a [u32], n: usize) -> Vec<&'a [u32]> {
    if n == 0 {
        return vec![nums];
    }
    assert!(nums.len() >= n, "invalid input"); //each partition should have atleast one element
    let mut res = Vec::new();
    let m = nums.len() / n; //no. of elements in each slice
    let mut start = 0;
    let mut end = m;
    loop {
        if end / m == n {
            //if last partition, it will have all the remaining elements (not necessarily m)
            res.push(&nums[start..]);
            break;
        } else {
            res.push(&nums[start..end]);
            start = end;
            end += m;
        }
    }
    res
}

fn main() {
    println!("{:?}", partition(&[1, 2, 3, 4, 5], 2));
    println!("{:?}", partition(&[12, 2, 34, 54, 6, 45, 6, 45, 5], 3));
    println!("{:?}", partition(&[7, 5, 4, 3, 6, 3, 5, 3, 6, 3, 6, 45], 5));
    println!("{:?}", partition(&[1, 2, 3, 4, 5, 6, 6, 7], 3));
}
