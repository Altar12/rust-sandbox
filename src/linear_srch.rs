//linear search

fn main() {
    let nums = [1, 2, 3, 5, 7, 22, 0];
    let key: i32;
    let mut input = String::with_capacity(5);
    let mut i = 0usize;
    let mut pos = 0usize;

    println!("Enter key to search");
    std::io::stdin()
        .read_line(&mut input)
        .expect("could not read input");
    input.pop();
    key = input.parse().expect("invalid input");

    while i < nums.len() {
        if nums[i] == key {
            pos = i + 1;
            break;
        }
        i += 1;
    }

    if pos > 0 {
        println!("Key found at position {pos}");
    } else {
        println!("Key not found");
    }
}
