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

use std::io::BufRead;
use std::collections::HashMap;

fn frekvens(fil: &str) -> Option<(String, usize)> {
    let fh = match std::fs::File::open(fil) {
	Ok(fh) => fh,
	Err(_) => return None,
    };
    let mut frek = HashMap::<String, usize>::new();
    for rad in std::io::BufReader::new(fh).lines() {
	if let Ok(rad) = rad {
	    for ord in rad.split_whitespace() {
		let ord = ord.to_lowercase();
		let p = frek.entry(ord).or_insert(0);
		*p += 1;
	    }
	} else {
	    return None;
	}
    }

    match frek.iter().max_by_key(|t| *t.1) {
	Some((namn, &p)) => Some((namn.clone(), p)),
	None => None,
    }
}


fn main() {
    println!("Ord: {:?}", frekvens("ordfil.txt"));
}




