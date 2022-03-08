/*  ex07a_keys  */
fn main() {
    let poäng = tabell("scores.txt");
    for namn in poäng.keys() {
	println!("{} fick {} poäng.", namn, poäng[namn]);
    }
}
// Ken fick 27 poäng.
// Bill fick 12 poäng.
// Steve fick 19 poäng.
// Linus fick 33 poäng.

use std::io::BufRead;
use std::collections::HashMap;

fn tabell(filnamn: &str) -> HashMap::<String, usize> {
    let mut ret = HashMap::new();
    if let Ok(fh) = std::fs::File::open(filnamn) {
	for rad in std::io::BufReader::new(fh).lines() {
	    if let Ok(rad) = rad {
		let v: Vec<&str> = rad.split_whitespace().collect();
		if v.len() != 2 { continue; }
		if let Ok(n) = v[1].parse::<usize>() {
		    ret.insert(v[0].to_string(), n);
		}
	    }
	}
    }
    ret
}





