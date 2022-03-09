/* Övning 602: kopiera

Skriv funktionen kopiera som tar
två filnamn som argument. Alla rader
från första filen som är på formatet
Bill 12
(dvs. en sträng följd av blanktecken
följd av ett icke-negativt heltal)
ska skrivas till den andra filen.

 */

use std::io::{BufRead, BufReader, Write};

fn kopiera(fil: &str, utfil: &str) -> std::io::Result<()> {
    let fh = std::fs::File::open(fil)?;
    let mut ufh = std::fs::File::create(utfil)?;
    for rad in BufReader::new(fh).lines() {
	let rad = rad?;
	let mut v = rad.split_whitespace();
	if let Some(_) = v.next() {
	    if let Some(tal) = v.next() {
		if let Ok(_) = tal.parse::<usize>() {
		    if v.next() == None {
			writeln!(ufh, "{}", rad)?;
		    }
		}
	    }
	}
    }
    ufh.sync_all()
}

fn main() {
    kopiera("scores.txt", "kopia.txt").unwrap();
}
