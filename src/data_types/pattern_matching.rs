use core::fmt;
use std::fmt::Display;

enum Message {
    Quit,
    Move(u8, u8),
    MoveS{x: u8, y: u8},
    Write(String),
    ChangeColor(i8, i8, i8),
    Introduce(Person)
}

struct Person {
    age: i8,
    name: String,
}

impl Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_> ) -> fmt::Result {
        writeln!(f, "age: {} - name: {}", self.age, self.name)
    }
}

pub fn start() {
    println!("Hello from module");
    send_messages();
} 

fn send_messages() {
    let ms1 = Message::Quit;
    let ms2 = Message::Move(4, 8);
    let ms3 = Message::MoveS{x:1, y:3};
    let ms4 = Message::Write(String::from("hello"));
    let ms5 = Message::ChangeColor(8, 88, 99);
    let ms6 = Message::Introduce(Person{age: 18, name: String::from("Alvaro")});

    crate::pattern_matching::handle_messages(ms1);
    crate::pattern_matching::handle_messages(ms2);
    crate::pattern_matching::handle_messages(ms3);
    crate::pattern_matching::handle_messages(ms4);
    crate::pattern_matching::handle_messages(ms5);
    crate::pattern_matching::handle_messages(ms6);
}

fn handle_messages(message: Message) {
    match message {
        Message::Quit => println!("quit"),
        Message::Move (x, y) => println!("move {} - {}", x, y),
        Message::MoveS {x, y} => println!("move_s {} - {}", x, y),
        Message::Write (s) => println!("Write: {}", s),
        Message::ChangeColor(_, _, z) => {
            let number_string = z.to_string();
            println!("{}", number_string);
        },
        Message::Introduce(p) => println!("{}", p)
    }
    
}
