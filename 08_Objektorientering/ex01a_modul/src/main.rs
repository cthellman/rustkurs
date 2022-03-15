/*  ex01a_modul  */

// Fil: main.rs
mod datum;      // The file must be named datum.rs. In sub-folders folder::module
use datum::*;   // Don't have to have this, but then use datum::Datum

fn main() {
    let idag = Datum {
	year: 2022, month: 3, day: 11
    };
    println!("{idag:?}");
}
// Datum { year: 2022, month: 3, day: 11 }

