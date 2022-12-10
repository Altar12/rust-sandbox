//Queue implementation with array, without using front and rear variables
const MAX_ELEMENTS: usize = 10;
const SIZE: usize = MAX_ELEMENTS + 1;
static mut NUMS: [Option<u32>; SIZE] = [None; SIZE];

unsafe fn front() -> Option<usize> {
    if let Some(_) = NUMS[0] {
        if let None = NUMS[SIZE - 1] {
            return Some(0);
        }
    }
    for i in 1..SIZE {
        if let Some(_) = NUMS[i] {
            if let None = NUMS[i - 1] {
                return Some(i);
            }
        }
    }
    None
}

unsafe fn rear() -> Option<usize> {
    for i in 0..SIZE {
        if let Some(_) = NUMS[i] {
            if let None = NUMS[(i + 1) % SIZE] {
                return Some(i);
            }
        }
    }
    None
}

unsafe fn element_cnt() -> usize {
    let mut res = 0;
    for i in 0..SIZE {
        if let Some(_) = NUMS[i] {
            res += 1;
        }
    }
    res
}

unsafe fn insert(ele: u32) -> Result<usize, ()> {
    match element_cnt() {
        MAX_ELEMENTS => return Err(()),
        0 => {
            NUMS[0] = Some(ele);
            Ok(0)
        }
        _ => {
            let last_pos = rear().unwrap();
            let insert_pos = (last_pos + 1) % SIZE;
            NUMS[insert_pos] = Some(ele);
            Ok(insert_pos)
        }
    }
}

unsafe fn remove() -> Result<u32, ()> {
    match front() {
        Some(i) => {
            let removed = NUMS[i].unwrap();
            NUMS[i] = None;
            Ok(removed)
        }
        None => Err(()),
    }
}

unsafe fn poll() -> Option<u32> {
    match front() {
        Some(i) => Some(NUMS[i].unwrap()),
        None => None,
    }
}

unsafe fn print() {
    if element_cnt() == 0 {
        return println!("EMPTY QUEUE");
    }
    let first_pos = front().unwrap();
    let last_pos = rear().unwrap();
    if first_pos <= last_pos {
        for i in first_pos..=last_pos {
            print!("{},", NUMS[i].unwrap());
        }
    } else {
        for i in first_pos..SIZE {
            print!("{},", NUMS[i].unwrap());
        }
        for i in 0..=last_pos {
            print!("{},", NUMS[i].unwrap());
        }
    }
    println!("");
    println!("^");
    println!("|");
    println!("front");
}

fn read(input: &mut String) {
    input.clear();
    std::io::stdin()
        .read_line(input)
        .expect("could not read input");
    input.pop();
}

fn main() {
    let mut input = String::new();
    unsafe {
        loop {
            println!("1.Insert\t2.Remove\t3.Poll\t4.Print Queue\t5.Exit");
            read(&mut input);
            match input.as_str() {
                "1" => {
                    println!("Enter element to insert");
                    read(&mut input);
                    match insert(input.parse().expect("parse error, check your input")) {
                        Err(_) => println!("Queue is full!"),
                        _ => (),
                    }
                }
                "2" => match remove() {
                    Err(_) => println!("Queue is empty!"),
                    _ => (),
                },
                "3" => match poll() {
                    Some(x) => println!("Next element in line: {x}"),
                    None => println!("Queue is empty!"),
                },
                "4" => print(),
                "5" => break,
                _ => println!("Invalid input!"),
            }
        }
    }
}
