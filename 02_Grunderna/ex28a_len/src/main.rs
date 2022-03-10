/*  ex28a_len  */
fn main() {
    let p = String::from("Göran");
    println!("Storlek: {}", p.len());           // UTF-8 makes ö two bytes
    println!("Längd: {}", p.chars().count());   // !!!Count chars!!! => use this
}
// Storlek: 6
// Längd: 5
