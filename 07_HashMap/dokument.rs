/*  ex01a_hashmap  */
use std::collections::HashMap;

fn main() {
    let mut res = HashMap::<usize, f64>::new();
    res.insert(2021, 1.7);
    res.insert(2020, -2.5);
    println!("Resultat: {:?}", res);
    println!("I fjol: {}", res[&2021]);
}
// Resultat: {2021: 1.7, 2020: -2.5}
// I fjol: 1.7




//********************************//


/*  ex02a_hashmap  */
use std::collections::HashMap;

fn main() {
    let mut poäng = HashMap::<&str, usize>::new();
    poäng.insert("Bill", 12);
    poäng.insert("Steve", 19);
    println!("Bill fick {}", poäng["Bill"]);
    println!("Steve fick {:?}", poäng.get("Steve"));
}
// Bill fick 12
// Steve fick Some(19)




//********************************//


/*  ex03a_pairs  */
use std::collections::HashMap;

fn main() {
    let a = [("Bill".to_string(), 12),
	     ("Steve".to_string(), 19)];
    let mut poäng = HashMap::from(a);
    println!("Ken finns: {}", poäng.contains_key("Ken"));
    if let Some(n) = poäng.remove("Bill") {
	println!("Bill hade {} poäng", n);
    }
    println!("Storlek {}", poäng.len());
}
// Ken finns: false
// Bill hade 12 poäng
// Storlek 1




//********************************//


/*  ex04a_get_mut  */
use std::collections::HashMap;

fn main() {
    let a = [("Bill".to_string(), 12), ("Steve".to_string(), 19)];
    let mut poäng = HashMap::from(a);
    if let Some(v) = poäng.get_mut("Steve") {
	*v += 3;
    }
    println!("Värde: {:?}", poäng);
}
// Värde: {"Steve": 22, "Bill": 12}



//********************************//


/*  ex05a_entry  */
use std::collections::HashMap;

fn main() {
    let a = [("Bill".to_string(), 12), ("Steve".to_string(), 19)];
    let mut poäng = HashMap::from(a);
    let p1 = poäng.entry("Steve".to_string()).or_insert(0);
    *p1 += 3;
    let p2 = poäng.entry("Ken".to_string()).or_insert(0);
    *p2 += 8;
    println!("Värde: {:?}", poäng);
}
// Värde: {"Ken": 8, "Steve": 22, "Bill": 12}



//********************************//


/*  ex06a_loop  */
fn main() {
    let poäng = tabell("scores.txt");
    for (namn, p) in &poäng {
	println!("{} fick {} poäng.", *namn, *p);
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



//********************************//


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



//********************************//


/*  ex08a_items  */
fn main() {
    let mut poäng = tabell("scores.txt");
    for (_namn, p) in &mut poäng {
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



//********************************//


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



//********************************//


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



//********************************//


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



//********************************//


/* Övning 701: frukter

Skapa funktionen kilopris. Den ska ta en referens
till en HashMap::<&str, f64> som parameter.
Nycklarna är namn på frukter, och värdena är kilopris.
Funktionen ska skriva ut frukterna, en per rad, sorterade
i växande ordning efter kilopriset. T.ex.
Banan kostar 25.00 kr
Päron kostar 29.75 kr

 */




//********************************//


/* Övning 702: ordfrekvens

Skapa funktionen frekvens. Den ska ta ett filnamn som
parameter. Funktionen ska räkna antalet förekomster
av varje ord i filen.

Funktionen ska returnera en option med en tupel av
typen (String, usize), t.ex. Some(("och", 14)), med
det mest frekventa ordet samt antalet förekomster.

Gör ingen skillnad mellan små och stora bokstäver
(använd metoden to_lowercase).

Ifall flera ord förekommer lika många gånger, så välj
något av dem.

Vid fel, eller om inga ord finns, returnera None.

*/


//********************************//


