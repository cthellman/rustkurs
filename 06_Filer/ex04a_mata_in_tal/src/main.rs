/*  ex04a_mata_in_tal  */
use std::io::prelude::*;

fn main() -> Result<(), std::io::Error> {
    print!("Skriv ett tal: ");
    std::io::stdout().flush()?;	// question mark returns if error occur
    for rad in std::io::stdin().lock().lines() {	// lock - lock to the current thread
	if let Ok(n) = rad?.parse::<usize>() {
	    println!("Du skrev {}", n);
	    break;
	}
	print!("Ej tal, försök igen: ");
	std::io::stdout().flush()?; // question mark returns if error occur
    }
    Ok(())
}
// Skriv ett tal: ew
// Ej tal, försök igen: 43
// Du skrev 43






