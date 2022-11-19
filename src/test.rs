struct Color(u8, u8, u8);
pub fn run() {
    let color_second = Color(255, 0, 0);
    println!("{}", color_second.0);
}
