/*  ex02a_hashmap  */
use std::collections::HashMap;

fn main() {
    let mut poäng = HashMap::<&str, usize>::new();
    poäng.insert("Bill", 12);
    poäng.insert("Steve", 19);
    println!("Bill fick {}", poäng["Bill"]);
    println!("Steve fick {:?}", poäng.get("Steve"));
}
// Bill fick 12
// Steve fick Some(19)






