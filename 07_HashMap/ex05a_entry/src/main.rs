/*  ex05a_entry  */
use std::collections::HashMap;

fn main() {
    let a = [("Bill".to_string(), 12), ("Steve".to_string(), 19)];
    let mut poäng = HashMap::from(a);
    let p1 = poäng.entry("Steve".to_string()).or_insert(0);
    *p1 += 3;
    let p2 = poäng.entry("Ken".to_string()).or_insert(0);
    *p2 += 8;
    println!("Värde: {:?}", poäng);
}
// Värde: {"Ken": 8, "Steve": 22, "Bill": 12}





