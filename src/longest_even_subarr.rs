fn contains_only_even(nums: &[i32]) -> bool {
    for num in nums {
        if *num % 2 == 1 {
            return false;
        }
    }
    true
}

fn longest_even_subarr(nums: &[i32]) -> &[i32] {
    let len = nums.len();
    let mut mut_len = nums.len();
    let mut res: &[i32];
    loop {
        if mut_len == 0 {
            break;
        }
        for start in 0..=(len - mut_len) {
            res = &nums[start..start + mut_len];
            if contains_only_even(res) {
                return res;
            }
        }
        mut_len -= 1;
    }
    &[][..]
}

#[cfg(test)]
#[test]
fn first() {
    let input = [1, 2, 4, 6, 7, 8, 9, 0];
    assert_eq!(longest_even_subarr(&input[..]), &input[1..4]);
}
#[test]
fn second() {
    let input = [1, 2, 3, 4, 5, 6];
    assert_eq!(longest_even_subarr(&input[..]), &input[1..2]);
}
#[test]
fn last() {
    let input = [];
    assert_eq!(longest_even_subarr(&input[..]), &[][..]);
}

fn main() {
    let str = String::from("hello there");
    let ref1 = &str;
    let ref2 = ref1;
    println!("{} {}", ref1, ref2);
}
