/*  ex08a_equality  */
#[derive(PartialEq)]
enum Färg {
    Klöver,
    Ruter,
    Hjärter,
    Spader,
}

fn main() {
    let s = Färg::Hjärter;
    if s == Färg::Klöver || s == Färg::Spader {
	println!("Svart kort.");
    } else {
	println!("Rött kort.");
    }
}
// Rött kort.
