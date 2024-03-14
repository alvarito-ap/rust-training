use std::io;

mod pattern_matching;
mod structs;

enum Selection {
    PatternMatching,
    Structs,
    None,
}

pub fn run_data_types() {
    let mut options = String::new();
    options.push_str("Pattern matching\n");
    options.push_str("Structs\n");
    println!("{}", options);

    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading data types option.");

    let selection = match input.trim() {
        "Pattern matching" => Selection::PatternMatching,
        "structs" => Selection::Structs,
        _ => Selection::None,
    };

    match selection {
        Selection::PatternMatching => pattern_matching::start(),
        Selection::Structs => structs::start(),
        _ => (),
    }
}
