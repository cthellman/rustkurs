/*  ex05a_display  */

// Fil: main.rs
mod datum;
use datum::Datum;

fn main() {
    let idag = Datum::new(2022, 3, 11).unwrap();
    let sedan = Datum::new(2024, 2, 15).unwrap();
    println!("idag: {}", idag);
    println!("sedan: {}", sedan.datum());
}
// idag: 11 mar 2022
// sedan: 2024-02-15

