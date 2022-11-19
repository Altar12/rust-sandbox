//Greater of two numbers

fn greater(x: i32, y: i32) -> Option<i32> {
    if x > y {
        Some(x)
    } else if y > x {
        Some(y)
    } else {
        None
    }
}

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
    let mut input = String::new();

    println!("Enter two nos:");
    read(&mut input);
    x = input.parse().expect("invalid input");
    read(&mut input);
    y = input.parse().expect("invalid input");

    match greater(x, y) {
        Some(num) => println!("Greater among {} & {} is {}", x, y, num),
        None => println!("Both numbers are equal"),
    }
}
