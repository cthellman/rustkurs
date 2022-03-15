/*  ex02a_konstruktor  */

// Fil: main.rs
mod datum;

fn main() {
    let idag = datum::Datum::new(2022, 3, 11).unwrap(); // unwrap undresses Some()
    println!("{:?}", idag);
}
// Datum { year: 2022, month: 3, day: 11 }

