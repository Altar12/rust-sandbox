// Implementation of a generic function to read values from standard input

use std::fmt::Debug;
use std::{io::stdin, str::FromStr};
fn read<T: FromStr>() -> T
where
    T::Err: Debug,
{
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Error reading input...");
    input.pop();
    T::from_str(&input).unwrap()
}

#[derive(Debug)]
struct Student {
    name: String,
    roll_no: u8,
    marks: f32,
}

impl Student {
    fn new(name: String, roll_no: u8, marks: f32) -> Self {
        Self {
            name,
            roll_no,
            marks,
        }
    }
}

fn main() {
    let name;
    let roll_no;
    let marks;
    println!("Enter student name, roll number, marks:");
    name = read();
    roll_no = read();
    marks = read();
    let student = Student::new(name, roll_no, marks);
    println!("{student:?}");
}
