//# design
fn main() {
    let row_cnt: u8;
    let sum: u8;
    let mut input = String::new();
    println!("Enter no. of rows: ");
    std::io::stdin()
        .read_line(&mut input)
        .expect("could not read input");
    input.pop();
    row_cnt = input.parse().unwrap();
    sum = row_cnt - 1;
    for i in 0..(row_cnt) {
        for _j in 0..(sum - i) {
            print!(" ");
        }
        for _j in 0..(2 * i + 1) {
            print!("#");
        }
        println!("");
    }
}
