fn prime_nums(start: u32, end: u32) -> Vec<u32> {
    let mut is_prime: Vec<bool> = Vec::with_capacity(end as usize + 1);
    let mut res: Vec<u32> = Vec::new();
    let mut j: usize;
    for _ in 0..end + 1 {
        is_prime.push(true);
    }
    for i in 2..is_prime.len() {
        if is_prime[i] {
            if i as u32 >= start {
                res.push(i as u32);
            }
            j = 2 * i;
            while j < is_prime.len() {
                is_prime[j] = false;
                j += i;
            }
        }
    }
    res
}

fn main() {
    println!("{:?}", prime_nums(1, 50));
    println!("{:?}", prime_nums(20, 50));
    println!("{:?}", prime_nums(100, 200));
    println!("{:?}", prime_nums(150, 300));
}
