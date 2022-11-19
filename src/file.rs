use std::fs::File;
use std::io::{ErrorKind, Write};

fn read(input: &mut String) {
    input.clear();
    std::io::stdin()
        .read_line(input)
        .expect("could not read input");
    input.pop();
}
fn main() {
    let mut file_name = String::new();
    let mut content = String::new();
    let mut file: File;

    println!("Enter file name:");
    read(&mut file_name);
    println!("Enter file contents:");
    read(&mut content);
    file = match File::open(file_name.clone()) {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create(file_name) {
                Ok(file) => file,
                Err(err) => panic!("encounterd err while creating file {}", err),
            },
            err => panic!("encountered err while opening file {}", err),
        },
    };
    match file.write(content.as_bytes()) {
        Ok(_) => println!("write successful"),
        _ => println!("write unsuccessful"),
    }
}
