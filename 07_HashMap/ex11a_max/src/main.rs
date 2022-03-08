/*  ex11a_max  */
fn main() {
    let poäng = tabell("scores.txt");
    let lista: Vec<(&String, &usize)> = poäng.iter().collect();
    let r = lista.iter().max_by_key(|t| *t.1);
    if let Some(par) = r {
	println!("{} {}", par.0, par.1);
    }
}
// Linus 33


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





