/*  ex04a_mata_in_tal  */
use std::io::prelude::*;

fn main() -> Result<(), std::io::Error> {
    print!("Skriv ett tal: ");
    std::io::stdout().flush()?;
    for rad in std::io::stdin().lock().lines() {
	if let Ok(n) = rad?.parse::<usize>() {
	    println!("Du skrev {}", n);
	    break;
	}
	print!("Ej tal, försök igen: ");
	std::io::stdout().flush()?;
    }
    Ok(())
}
// Skriv ett tal: ew
// Ej tal, försök igen: 43
// Du skrev 43






