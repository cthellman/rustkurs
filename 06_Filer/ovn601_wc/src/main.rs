/* Övning 601: wc

Skriv funktionen wc som tar ett
filnamn som parameter och som
returnerar en tupel av tre usize.
De tre talen ska vara antalet
rader, antalet ord och antalet
tecken i filen. Tupeln ska vara
inklädd i ett std::io::Result.

Multibyte-tecken som t.ex. 'å'
ska räknas som ett tecken.
Räkna inte med radbrytningar.

 */

use std::io::{BufReader, BufRead};

fn wc(fil: &str) -> std::io::Result<(usize, usize, usize)> {
    let fh = std::fs::File::open(fil)?;
    let reader = BufReader::new(fh);
    let mut c = (0, 0, 0);
    for rad in reader.lines() {
	let rad = rad?;
	c.0 += 1;
	c.1 += rad.split_whitespace().count();
	c.2 += rad.chars().count();
    }
    Ok(c)
}

fn main() {
    println!("scores.txt: {:?}", wc("scores.txt"));
    println!("ordfil.txt: {:?}", wc("ordfil.txt"));
}
