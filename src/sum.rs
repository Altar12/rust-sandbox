fn read(input: &mut String) {
    input.clear();
    std::io::stdin()
        .read_line(input)
        .expect("could not read input");
    input.pop();
}

fn main() {
    let x: i32;
    let y: i32;
    let sum: i32;
    let mut input = String::new();
    println!("Enter two nos:");
    read(&mut input);
    x = input.parse().expect("invalid input");
    read(&mut input);
    y = input.parse().expect("invalid input");
    sum = x + y;
    println!("{} + {} = {}", x, y, sum);
}
