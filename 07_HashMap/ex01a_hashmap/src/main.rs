/*  ex01a_hashmap  */
use std::collections::HashMap;

fn main() {
    let mut res = HashMap::<usize, f64>::new();
    res.insert(2021, 1.7);
    res.insert(2020, -2.5);
    println!("Resultat: {:?}", res);
    println!("I fjol: {}", res[&2021]);
}
// Resultat: {2021: 1.7, 2020: -2.5}
// I fjol: 1.7






