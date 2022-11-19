//Vowel or consonant

fn main() {
    let ch: char;
    let mut input = String::with_capacity(2);
    println!("Enter an alphabet");
    std::io::stdin()
        .read_line(&mut input)
        .expect("could not read input");
    input.pop();
    ch = input.parse().expect("invalid input");

    if ch == 'a'
        || ch == 'e'
        || ch == 'i'
        || ch == 'o'
        || ch == 'u'
        || ch == 'A'
        || ch == 'E'
        || ch == 'I'
        || ch == 'O'
        || ch == 'U'
    {
        println!("You entered a vowel");
    } else if ch > 'A' && ch <= 'Z' || ch > 'a' && ch <= 'z' {
        println!("You entered a consonant");
    } else {
        println!("You did not enter an alphabet");
    }
}
