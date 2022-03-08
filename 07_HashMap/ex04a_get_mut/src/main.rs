/*  ex04a_get_mut  */
use std::collections::HashMap;

fn main() {
    let a = [("Bill".to_string(), 12), ("Steve".to_string(), 19)];
    let mut poäng = HashMap::from(a);
    if let Some(v) = poäng.get_mut("Steve") {
	*v += 3;
    }
    println!("Värde: {:?}", poäng);
}
// Värde: {"Steve": 22, "Bill": 12}





