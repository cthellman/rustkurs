/*  ex03a_pairs  */
use std::collections::HashMap;

fn main() {
    let a = [("Bill".to_string(), 12),
	     ("Steve".to_string(), 19)];
    let mut poäng = HashMap::from(a);
    println!("Ken finns: {}", poäng.contains_key("Ken"));
    if let Some(n) = poäng.remove("Bill") {
	println!("Bill hade {} poäng", n);
    }
    println!("Storlek {}", poäng.len());
}
// Ken finns: false
// Bill hade 12 poäng
// Storlek 1






