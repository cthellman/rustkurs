/* Övning 301: Gissa tal

Välj ett slumptal mellan 1 och 1000. Låt användaren
gissa vilket talet är. Om gissningen är för liten
eller för stor, så ska programmet säga det och sedan
låta användaren gissa igen. Ifall gissningen är rätt
ska programmet berätta det och sedan avslutas. T.ex.

Gissa tal (1-1000): 500
För litet. Gissa tal (1-1000): kalle
Ej tal! Gissa tal (1-1000): 600
För stort. Gissa tal (1-1000): 510
För stort. Gissa tal (1-1000): 505
Rätt efter 4 gissningar!

*/

use std::io::{stdin, stdout, Write};

fn main() {
    let svar = rand::random::<usize>() % 1000 + 1;
    let mut n = 0;
    loop {
	print!("Gissa tal (1-1000): ");
	stdout().flush().expect("Kan inte skriva till stdout");
	let mut g = String::new();
	stdin().read_line(&mut g).expect("Kan inte läsa från stdin");
	match g.trim().parse::<usize>() {
	    Ok(gissning) => {
		n += 1;
		if gissning == svar { break }
		if gissning < svar {
		    print!("För litet. ");
		} else {
		    print!("För stort. ");
		}
	    } Err(_) => {
		if g.trim().is_empty() { return; }
		print!("Ej tal! ");
	    }
	}
    }
    println!("Rätt efter {n} gissningar!");
}
