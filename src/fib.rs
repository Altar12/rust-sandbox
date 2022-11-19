pub fn run() {
    let input = 10;
    assert_eq!(input > 0, true);
    let (mut a, mut b, mut c) = (1, 0, 0);
    let mut i = 2;
    println!("0");
    while i <= input {
        c = a + b;
        a = b;
        b = c;
        println!("{}", c);
        i += 1;
    }
}
