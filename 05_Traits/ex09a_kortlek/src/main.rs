/*  ex09a_kortlek  */
#[derive(PartialEq)]
enum Färg {
    Klöver(u8),
    Ruter(u8),
    Hjärter(u8),
    Spader(u8),
}

use std::cmp::Ordering;
use Färg::*;

impl PartialOrd for Färg {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
	    (Klöver(m), Klöver(n)) => Some(m.cmp(n)),
	    (Ruter(m), Ruter(n)) => Some(m.cmp(n)),
	    (Hjärter(m), Hjärter(n)) => Some(m.cmp(n)),
	    (Spader(m), Spader(n)) => Some(m.cmp(n)),
	    _ => None,
	}
    }
}

fn main() {
    let s = Hjärter(5);
    let t = Hjärter(13);
    println!("{:?}", s.partial_cmp(&t));
    println!("{:?}", s.partial_cmp(&Klöver(4)));
}
// Some(Less)
// None






