use std::io::Write;

fn run() {
    let cnt: u8;
    let mut input = String::new();
    let (mut a, mut b, mut c) = (1u32, 0u32, 0u32);
    print!("how many fibonacci digits to generate:");
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut input)
        .expect("could not read input");
    input.pop();
    cnt = input.parse().unwrap();
    assert_eq!(cnt > 0, true);
    print!("0");
    for _i in 1..cnt {
        c = a + b;
        print!(",{}", c);
        a = b;
        b = c;
    }
    println!("");
}
