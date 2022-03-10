/*  ex19a_felhantering  */
use std::io::{stdin, stdout, Write};    // using syntax

fn main() {
    let mut namn = String::new();
    print!("Vad heter du? ");
    stdout().flush().expect("Misslyckades");
    match stdin().read_line(&mut namn) {
	Ok(_n) => println!("Hej, {}!", namn.trim()),    // don't care about the ok-value, start with underscore
	Err(_fel) => println!("Misslyckades"),
    }
}


