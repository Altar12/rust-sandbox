struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    fn update_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

pub fn run() {
    let mut person = Person::new("Shubham", "Singh");
    println!("{}", person.full_name());
    person.update_last_name("Uzumaki");
    println!("{}", person.full_name());
}
