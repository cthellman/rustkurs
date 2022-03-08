/*  ex10a_sortera  */
fn main() {
    let poäng = tabell("scores.txt");
    let mut lista: Vec<_> = poäng.iter().collect();
    lista.sort_by_key(|t| -(*t.1 as isize));
    for (namn, p) in &lista {
	print!("{} {} ", namn, p);
    }
}
// Linus 33 Ken 27 Steve 19 Bill 12


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





