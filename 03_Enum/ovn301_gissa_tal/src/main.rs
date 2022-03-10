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
use rand::Rng;
use std::io::{stdin, stdout, Write};

fn main() {

    let num:i32 = rand::thread_rng().gen_range(1..1000);
    loop {
	print!("Gissa tal (1-1000)?");
	stdout().flush().expect("Misslyckades");
	let mut guess = String::new();
	match stdin().read_line(&mut guess) {
	    Ok(n) => if n == 0 { break; },
	    Err(_) => {
		print!("Förstår ej, försök igen");
		continue;
	    }
	}
	match guess.trim().parse::<i32>() {
	    Ok(n) => {
		if n == num {
		    println!("Rätt!");
		    break;
		}
        else if n > num {
            println!("För högt, försök igen!");
        }
        else if n < num {
            println!("För lågt, försök igen!");
        }
	    } Err(_) => {
		print!("Ej tal, försök igen! ");
	    }
	}
    }



}
