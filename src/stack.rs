fn read(input: &mut String) {
    input.clear();
    std::io::stdin()
        .read_line(input)
        .expect("could not read input");
    input.pop();
}

enum StackError {
    EmptyStack,
    FullStack,
}

struct Stack {
    nums: [u32; 10],
    ele_cnt: usize,
}

impl Stack {
    fn new() -> Self {
        Self {
            nums: [0u32; 10],
            ele_cnt: 0,
        }
    }

    fn push(&mut self, element: u32) -> Result<(), StackError> {
        if self.ele_cnt == 10 {
            Err(StackError::FullStack)
        } else {
            let index = self.ele_cnt;
            self.nums[index] = element;
            self.ele_cnt += 1;
            Ok(())
        }
    }

    fn pop(&mut self) -> Result<u32, StackError> {
        if self.ele_cnt == 0 {
            Err(StackError::EmptyStack)
        } else {
            self.ele_cnt -= 1;
            let index = self.ele_cnt;
            Ok(self.nums[index])
        }
    }

    fn peak(&mut self) -> Result<u32, StackError> {
        if self.ele_cnt == 0 {
            Err(StackError::EmptyStack)
        } else {
            let index = self.ele_cnt - 1;
            Ok(self.nums[index])
        }
    }
}

fn run() {
    let mut my_stack = Stack::new();
    let mut input = String::new();
    let mut ele: u32;
    let mut size: usize;
    loop {
        size = my_stack.ele_cnt;
        println!("{:?}", &my_stack.nums[..size]);
        println!("1.Push\t2.Pop\t3.Peak\t4.Exit");
        read(&mut input);
        match input.as_str() {
            "1" => {
                read(&mut input);
                ele = input.parse().unwrap();
                if let Err(_) = my_stack.push(ele) {
                    println!("stack full!");
                }
            }
            "2" => {
                if let Ok(x) = my_stack.pop() {
                    println!("popped {}!", x);
                } else {
                    println!("stack empty!");
                }
            }
            "3" => {
                if let Ok(x) = my_stack.peak() {
                    println!("{} at top", x);
                } else {
                    println!("stack empty!");
                }
            }
            "4" => break,
            _ => println!("invalid input"),
        }
    }
}
