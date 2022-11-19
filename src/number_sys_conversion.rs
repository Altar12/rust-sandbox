//converting from one number system to another
fn is_valid_num(num: &str, base: u8) -> bool {
    let end1;
    let end2;
    if base <= 10 {
        end1 = (47 + base) as char;
        end2 = end1;
    } else {
        end1 = '9';
        end2 = (54 + base) as char;
    }
    for ch in num.chars() {
        if !(ch >= '0' && ch <= end1 || ch >= 'A' && ch <= end2) {
            return false;
        }
    }
    true
}

fn convert_to_decimal(num: &str, base: u8) -> u32 {
    let mut multiplier = 1;
    let mut res = 0;
    let mut digit;
    let base = base as u32;
    for _ in 1..num.len() {
        multiplier *= base;
    }
    for ch in num.chars() {
        digit = if ch <= '9' {
            ch as u32 - 48
        } else {
            ch as u32 - 55
        };
        res += digit * multiplier;
        multiplier /= base;
    }
    res
}

fn convert_from_decimal(mut num: u32, base: u8) -> String {
    let mut res = String::new();
    let mut mods: Vec<char> = Vec::new();
    let base = base as u32;
    let mut digit;
    let mut remainder;
    loop {
        remainder = num % base;
        digit = match remainder as u8 {
            x @ 0..=9 => (48 + x) as char,
            x => (55 + x) as char,
        };
        mods.push(digit);
        num /= base;
        if num == 0 {
            break;
        }
    }
    while mods.len() > 0 {
        res.push(mods.pop().unwrap());
    }
    res
}

fn read(input: &mut String) {
    input.clear();
    std::io::stdin()
        .read_line(input)
        .expect("could not read input");
    input.pop();
}

fn main() {
    let base1: u8;
    let base2: u8;
    let mut input = String::new();
    let intermediate;
    let res;
    println!("Enter input & output base");
    read(&mut input);
    base1 = input.parse().expect("invalid input");
    read(&mut input);
    base2 = input.parse().expect("invalid input");
    if base1 < 2 || base1 > 36 || base2 < 2 || base2 > 36 {
        println!("Invalid base entered");
        println!("Allowed base range: 2..=36");
        return ();
    }
    println!("Enter number in base{base1}");
    read(&mut input);
    if !is_valid_num(&input, base1) {
        println!("Invalid number entered");
        return ();
    }
    intermediate = convert_to_decimal(&input, base1);
    res = convert_from_decimal(intermediate, base2);
    println!("Output number: {res}");
}
