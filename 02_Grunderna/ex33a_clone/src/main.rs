/*  ex33a_clone  */
fn main() {
    let namn = String::from("Steve");
    let p = namn.clone();   // if assigned, ownership had been moved and namn would be unusable after this...
    println!("Namn: {} och {}", namn, p);
}
// Namn: Steve och Steve
