use std::{io, u8};

mod collections;
mod data_types;
mod lifetimes;

fn main() {
    loop {
        let mut select_menu: String = String::new();
        select_menu.push_str("1: Data types\n");
        select_menu.push_str("2: Collections\n");
        select_menu.push_str("0: Exit\n");
        println!("{}", select_menu);

        let mut input: String = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Error reading line");

        match input.trim().parse::<u8>().unwrap_or(0) {
            1 => data_types::run_data_types(),
            2 => collections::run_collections(),
            0 => {
                println!("Exit");
                break;
            }
            _ => continue,
        }
    }
}
