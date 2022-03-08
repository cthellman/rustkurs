/*  ex13a_parse  */
fn main() {
    let tal = String::from("215");
    if let Ok(n) = tal.parse::<usize>() {
	println!("Talet är {}", n);
    } else {
	println!("Ej tal.");
    }
}
// Talet är 215
