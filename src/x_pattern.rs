fn print_x(half_ht: u32, ch: char) {
    let max_space = 2 * half_ht - 1;
    for i in 0..half_ht {
        for _ in 0..i {
            print!(" ");
        }
        print!("{ch}");
        for _ in 0..max_space - 2 * i {
            print!(" ");
        }
        println!("{ch}");
    }
    for _ in 0..half_ht {
        print!(" ");
    }
    println!("{ch}");
    let mut i = half_ht;
    while i > 0 {
        i -= 1;
        for _ in 0..i {
            print!(" ");
        }
        print!("{ch}");
        for _ in 0..max_space - 2 * i {
            print!(" ");
        }
        println!("{ch}");
    }
}

fn main() {
    print_x(6, '*');
}
