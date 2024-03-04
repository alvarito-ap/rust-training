use std::fmt::Display;

struct Person {
    age: u8,
    name: String,
}

impl Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.age, self.name)
    }
}
pub fn start() {
    let age = 25;
    let name = String::from("Alvaro");
    let p1 = Person{age, name};
    println!("{}", p1);
}
