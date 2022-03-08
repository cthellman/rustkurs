/*  ex03a_metod  */

// Fil: main.rs
mod datum;
use datum::Datum;

fn main() {
    let idag = Datum::new(2022, 3, 10).unwrap();
    let sedan = Datum::new(2024, 2, 15).unwrap();
    println!("Skottår 2022: {}", idag.skottår());
    println!("Skottår 2024: {}", sedan.skottår());
}
// Skottår 2022: false
// Skottår 2024: true

