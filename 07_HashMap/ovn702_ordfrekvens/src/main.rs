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
use std::{io::{BufReader, BufRead}, collections::HashMap};

fn main() {
    println!("{:?}", läsafil("ordfil.txt").unwrap());
}

fn läsafil(fil: &str) -> Option<(String,usize)>{

    let mut hash_ord = HashMap::<String, usize>::new();
    if let Ok(öppna) = std::fs::File::open(fil) {
        for buf in BufReader::new(öppna).lines()
        {
                if let Ok(buf) = buf {
                for ord in buf.split_whitespace(){
                    let entry = hash_ord.entry(ord.to_lowercase()).or_insert(0);
                    *entry += 1; 

                    // println!("{} {}",ord.to_lowercase(), entry);
                }
            }
        }
    }

    let max_ord = hash_ord.iter().max_by_key(|o| o.1).unwrap();
    // let ord_nyckel = max_ord.0.to_string();
    // let ord_siffra = *max_ord.1;
    Some((max_ord.0.clone(),*(max_ord.1)))
    // Some((ord_nyckel, ord_siffra))
}
