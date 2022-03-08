/*  ex06a_operator  */

// Fil: main.rs
mod datum;
use datum::Datum;

fn main() {
    let mut dag = Datum::new(2022, 3, 11).unwrap();
    dag.dagar_framåt(1);
    println!("imorgon: {}", dag);
    println!("sedan: {}", dag + 730);
}
// imorgon: 12 mar 2022
// sedan: 11 mar 2024

