//Cosine calculation with cosine series
const TERM_COUNT: u32 = 100;
fn cos(x: f64) -> f64 {
    let mut num = 1.0;
    let mut den = 1.0;
    let mut sign = 1.0;
    let mut res = 1.0;
    for i in 1..TERM_COUNT {
        sign = -sign;
        num *= x * x;
        den *= (2 * i * (2 * i - 1)) as f64;
        res += sign * num / den;
    }
    res
}

fn main() {
    let angle;
    let cos_val;
    let mut input = String::new();
    println!("Enter angle in radians");
    std::io::stdin()
        .read_line(&mut input)
        .expect("could not read input");
    input.pop();
    angle = input.parse().expect("parse error, check your input");
    cos_val = cos(angle);
    println!("cos{angle} = {cos_val}");
}
