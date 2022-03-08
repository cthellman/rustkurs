/*  ex09a_values  */
fn main() {
    let mut poäng = tabell("scores.txt");
    for p in poäng.values_mut() {
	*p += 5;
    }
    println!("Värde: {:?}", poäng);
}
// Värde: {"Steve": 24, "Bill": 17, "Linus": 38, "Ken": 32}


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





