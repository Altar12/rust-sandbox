//Linked list (prepend, remove, print)
enum Node {
    Val(i32, Box<Node>),
    Nil,
}
impl Node {
    fn prepend(self, data: i32) -> Self {
        Self::Val(data, Box::new(self))
    }
    fn remove(self) -> (Option<i32>, Self) {
        match self {
            Self::Val(data, rest) => (Some(data), *rest),
            Self::Nil => (None, self),
        }
    }
    fn print(&self) {
        match self {
            Self::Val(data, rest) => {
                print!("{data},");
                rest.print();
            }
            Self::Nil => println!("Nil"),
        }
    }
}

fn main() {
    let mut list = Node::Nil;
    let mut data;
    let mut input = String::new();
    loop {
        println!("1. Prepend\t2. Remove\t3. Print\t4. Exit");
        input.clear();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Could not read input");
        input.pop();
        match input.as_str() {
            "1" => {
                println!("Enter data to prepend");
                input.clear();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("Could not read input");
                input.pop();
                data = input.parse().expect("Parse error, check your input");
                list = list.prepend(data);
            }
            "2" => match list.remove() {
                (Some(x), rest) => {
                    println!("Removed {x}");
                    list = rest;
                }
                (None, rest) => {
                    println!("No data to remove");
                    list = rest;
                }
            },
            "3" => list.print(),
            "4" => break,
            _ => println!("Invalid input!"),
        }
    }
}
