/*  ex09a_kortlek  */
enum Färg {
    Klöver(u8),
    Ruter(u8),
    Hjärter(u8),
    Spader(u8),
}

use Färg::*;

fn main() {
    let s = Hjärter(5);
    match s {
	Hjärter(n) => println!("Hjärter {}", n),
	Ruter(n) => println!("Ruter {}", n),
	_ => println!("Svart kort."),
    }
}
// Hjärter 5
