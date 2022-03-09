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


fn main() {
    println!("scores.txt: {:?}", wc("scores.txt"));
    println!("ordfil.txt: {:?}", wc("ordfil.txt"));
}
