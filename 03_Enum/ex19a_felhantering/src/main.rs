/*  ex19a_felhantering  */
use std::io::{stdin, stdout, Write};

fn main() {
    let mut namn = String::new();
    print!("Vad heter du? ");
    stdout().flush().expect("Misslyckades");
    match stdin().read_line(&mut namn) {
	Ok(_n) => println!("Hej, {}!", namn.trim()),
	Err(_fel) => println!("Misslyckades"),
    }
}


