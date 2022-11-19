fn hcf(num1: u32, num2: u32) -> u32 {
    let (small, big) = if num1 >= num2 {
        (num2, num1)
    } else {
        (num1, num2)
    };
    if big % small == 0 {
        return small;
    }
    let mut res = 1;
    for i in 2..=small / 2 {
        if small % i == 0 && big % i == 0 {
            res = i;
        }
    }
    res
}
fn main() {
    println!(
        "{} {} {} {}",
        hcf(10, 15),
        hcf(15, 10),
        hcf(24, 30),
        hcf(20, 25)
    );
}
