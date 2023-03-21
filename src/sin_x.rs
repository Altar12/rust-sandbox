/*
    sin(x) calcualtion with sin series
    sin(x) = x - (x^3)/3! + (x^5)/5! - (x^7)/7! + ...
    x is specified in radians
*/
use std::ops::MulAssign;

const TERMS: u32 = 6;
const PI: f64 = 3.14159;

fn factorial(n: u32) -> u32 {
    let mut res = 1;
    for i in 2..=n {
        res *= i;
    }
    res
}

fn pow<T: MulAssign + Copy>(base: T, exponent: u32) -> T {
    let mut res = base;
    for _ in 1..exponent {
        res *= base;
    }
    res
}

fn sin(x: f64) -> f64 {
    let mut sum = 0.0;
    let mut sign = 1.0;
    for i in 0..TERMS {
        sum += sign * pow(x, 2 * i + 1) / factorial(2 * i + 1) as f64;
        sign = -sign;
    }
    sum
}

fn main() {
    println!(
        "sin(PI)={}, sin(PI/2)={}, sin(PI/4)={}, sin(0)={}",
        sin(PI),
        sin(PI / 2.0),
        sin(PI / 4.0),
        sin(0.0)
    );
}
