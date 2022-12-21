//Converting seconds to hh:mm:ss notation
fn main() {
    let mut seconds;
    let mut time = String::new();
    let mut input = String::new();
    let hours;
    let mins;
    println!("Enter no. of seconds");
    std::io::stdin()
        .read_line(&mut input)
        .expect("could not read input");
    input.pop();
    seconds = input
        .parse::<u128>()
        .expect("parse error, check your input");
    hours = seconds / 3600;
    seconds %= 3600;
    if hours < 10 {
        time.push('0');
    }
    time.push_str(&hours.to_string());
    time.push(':');
    mins = seconds / 60;
    seconds %= 60;
    if mins < 10 {
        time.push('0');
    }
    time.push_str(&mins.to_string());
    time.push(':');
    if seconds < 10 {
        time.push('0');
    }
    time.push_str(&seconds.to_string());
    println!("{time}");
}
