/*  ex10a_if_let  */
#[derive(PartialEq)]
enum Färg {
    Klöver(u8),
    Ruter(u8),
    Hjärter(u8),
    Spader(u8),
}

use Färg::*;

fn main() {
    let s = Hjärter(5);
    if let Hjärter(n) = s {
	println!("Hjärter {}", n);
    }
}
// Hjärter 5
