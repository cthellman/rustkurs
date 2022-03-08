/*  ex06a_vinnare  */
use std::io::{BufReader, BufRead};

fn vinnare(fil: &str) -> std::io::Result<String> {
    let fh = std::fs::File::open(fil)?;
    let mut max_p = 0;
    let mut ledare = String::new();
    for rad in BufReader::new(fh).lines() {
	let rad = rad?;
	let mut v = rad.split_whitespace();
	if let Some(namn) = v.next() {
	    if let Some(tal) = v.next() {
		if let Ok(n) = tal.parse::<usize>() {
		    if n >= max_p {
			max_p = n;
			ledare = namn.to_string();
		    }
		}
	    }
	}
    }
    Ok(ledare)
}

fn main() {
    if let Ok(namn) = vinnare("scores.txt") {
	println!("Vinnare: {}", namn);
    }
}
// Vinnare: Linus



