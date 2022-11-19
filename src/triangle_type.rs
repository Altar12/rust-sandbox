enum Triangle {
    Equilateral,
    Isosceles,
    Scalene,
}

fn triangle_type(x: u8, y: u8, z: u8) -> Result<Triangle, ()> {
    if x >= y + z || y >= x + z || z >= x + y {
        return Err(());
    }
    if x == y && x == z {
        Ok(Triangle::Equilateral)
    } else if x == y || y == z || x == z {
        Ok(Triangle::Isosceles)
    } else {
        Ok(Triangle::Scalene)
    }
}

fn read(input: &mut String) {
    input.clear();
    std::io::stdin()
        .read_line(input)
        .expect("could not read input");
    input.pop();
}

fn main() {
    let x: u8;
    let y: u8;
    let z: u8;
    let mut input = String::with_capacity(10);
    println!("Enter trianlge sides:");
    read(&mut input);
    x = input.parse().expect("invalid input");
    read(&mut input);
    y = input.parse().expect("invalid input");
    read(&mut input);
    z = input.parse().expect("invalid input");
    match triangle_type(x, y, z) {
        Err(_) => println!("The sides don't make up a triangle"),
        Ok(t) => match t {
            Triangle::Equilateral => println!("The sides make up an equilateral triangle"),
            Triangle::Isosceles => println!("The sides make up an isosceles triangle"),
            Triangle::Scalene => println!("The sides make up a scalene triangle"),
        },
    }
}
