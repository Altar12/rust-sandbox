fn get_words(input: &str) -> Vec<&str> {
    let mut res: Vec<&str> = Vec::with_capacity(10);
    let mut index: usize = 0;
    let mut start: usize = 0;
    for ch in input.as_bytes() {
        if *ch == b' ' {
            res.push(&input[start..index]);
            start = index + 1;
        }
        index += 1;
    }
    res.push(&input[start..]);
    res
}

fn main() {
    let input = "hello how are you? I am fine. Thank you!";
    let words = get_words(input);
    println!("{:?}", words);
}
