/*  ex20a_addera  */
use std::io::{stdin, stdout, Write};

fn main() {
    let a = rand::random::<usize>() % 10 + 1;
    let b = rand::random::<usize>() % 10 + 1;
    loop {
	print!("Vad är {} + {}? ", a, b);
	stdout().flush().expect("Misslyckades");
	let mut svar = String::new();
	match stdin().read_line(&mut svar) {
	    Ok(n) => if n == 0 { break; },
	    Err(_) => {
		print!("Förstår ej, försök igen");
		continue;
	    }
	}
	match svar.trim().parse::<usize>() {
	    Ok(n) => {
		if n == a + b {
		    println!("Rätt!");
		    break;
		}
		print!("Fel, försök igen! ");
	    } Err(_) => {
		print!("Ej tal, försök igen! ");
	    }
	}
    }
}
// Vad är 6 + 4? 11
// Fel, försök igen! Vad är 6 + 4? 10
// Rätt!

