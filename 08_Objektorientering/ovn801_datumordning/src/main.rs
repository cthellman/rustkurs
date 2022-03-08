/* Övning 801: datumordning

1. Verifiera att icke-ascii-identifierare fungerar.
   (Bl.a. krävs Rust 1.53 eller senare.)
   Om inte, ändra namn på lämpliga ställen
   så att det fungerar.

2. Implementera metoden dag så att den returnerar
   self.day. Och motsvarande för månad och år.

3. Implementera Ord och PartialOrd för Datum.

4. Ändra returtyp för new till Result<Datum, DatumFel>
*/

// Fil: main.rs
mod datum;
use datum::Datum;

fn main() {
    let data = [(2022, 3, 11), (2000, 1, 1), (1999, 12, 31),
		(2024, 3, 11), (1970, 1, 1), (2022, 12, 24)];
    let mut v: Vec<_> = data.iter().map(|&(y, m, d)|
	Datum::new(y as u16, m as u8, d as u8).unwrap()
    ).collect();
    println!("{} {} {}", v[0].år(), v[0].månad(), v[0].dag());
    println!("{} < {}: {}", v[0], v[1].datum(), v[0] < v[1]);
    v.sort();
    for d in v {
	print!("{} ", d);
    }
    println!();
}
// 2022 3 11
// 11 mar 2022 < 2000-01-01: false
// 1 jan 1970 31 dec 1999 1 jan 2000 11 mar 2022 24 dec 2022 11 mar 2024
