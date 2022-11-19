fn fact(num: u32) -> u32 {
    let mut res = 1u32;
    let mut x = 2u32;
    wfn fact(num: u32) -> u32 {
        let mut res = 1u32;
        let mut x = 2u32;
        fn fact(num: u32) -> u32 {
            let mut res = 1u32;
            let mut x = 2u32;
            while x <= num {
                res *= x;
                x += 1;
            }
            res
        }
        
        fn read(input: &mut String) {
            input.clear();
            std::io::stdin()
                .read_line(input)
                .expect("could not read input");
            input.pop();
        }
        
        fn main() {
            let num: u32;
            let res: u32;
            let mut input = String::with_capacity(5);
            println!("Enter a number");
            read(&mut input);
            num = input.parse().expect("invalid input");
            res = fact(num);
            println!("{num}! = {res}");
        }   while x <= num {
            res *= x;
            x += 1;
        }
        res
    }
    
    fn read(input: &mut String) {
        input.clear();
        std::io::stdin()
            .read_line(input)
            .expect("could not read input");
        input.pop();
    }
    
    fn main() {
        let num: u32;
        let res: u32;
        let mut input = String::with_capacity(5);
        println!("Enter a number");
        read(&mut input);
        num = input.parse().expect("invalid input");
        res = fact(num);
        println!("{num}! = {res}");
    }hile x <= num {
        res *= x;
        x += 1;
    }
    res
}

fn read(input: &mut String) {
    input.clear();
    std::io::stdin()
        .read_line(input)
        .expect("could not read input");
    input.pop();
}

fn main() {
    let num: u32;
    let res: u32;
    let mut input = String::with_capacity(5);
    println!("Enter a number");
    read(&mut input);
    num = input.parse().expect("invalid input");
    res = fact(num);
    println!("{num}! = {res}");
}
