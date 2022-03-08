/*  ex01a_modul  */

// Fil: main.rs
mod datum;

fn main() {
    let idag = datum::Datum {
	year: 2022, month: 3, day: 11
    };
    println!("{idag:?}");
}
// Datum { year: 2022, month: 3, day: 11 }

