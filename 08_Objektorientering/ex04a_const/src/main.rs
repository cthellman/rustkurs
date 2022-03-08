/*  ex04a_const  */

// Fil: main.rs
mod datum;
use datum::Datum;

fn main() {
    let idag = Datum::new(2022, 3, 10).unwrap();
    let sedan = Datum::new(2024, 2, 15).unwrap();
    println!("Dagar: {}", idag.månadslängd());
    println!("Dagar: {}", sedan.månadslängd());
}
// Dagar: 31
// Dagar: 29

