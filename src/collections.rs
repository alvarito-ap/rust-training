use std::io;

mod vectors;

enum Collections {
    Vectors,
    None,
}

pub fn run_collections() {
    let mut options = String::new();
    options.push_str("Vectors\n");
    println!("{}", options);

    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading collections input.");
    let selection: Collections = match input.trim() {
        "Vectors" => Collections::Vectors,
        _ => Collections::None,
    };

    match selection {
        Collections::Vectors => vectors::run_vectors(),
        _ => (),
    }
}
