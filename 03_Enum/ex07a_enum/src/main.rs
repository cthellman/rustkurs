/*  ex07a_enum  */
enum Färg {
    Klöver,
    Ruter,
    Hjärter,
    Spader,
}

fn main() {
    let s = Färg::Hjärter;
    match s {
	Färg::Hjärter | Färg::Ruter => {
	    println!("Rött")
	}
	_ => println!("Svart")
    }
}
// Rött
