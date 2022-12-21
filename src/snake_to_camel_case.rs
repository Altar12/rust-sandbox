//Converting snake case to camel case
fn main() {
    let mut input = String::new();
    let mut output = String::new();
    let mut capitalize = false;
    let diff = 'a' as u8 - 'A' as u8;
    println!("Enter string in snake case");
    std::io::stdin()
        .read_line(&mut input)
        .expect("could not read input");
    input.pop();
    for ch in input.chars() {
        if capitalize {
            output.push((ch as u8 - diff) as char);
            capitalize = false;
        } else if ch == '_' {
            capitalize = true;
        } else {
            output.push(ch);
        }
    }
    println!("Camel case: {output}");
}
