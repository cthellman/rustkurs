/*  ex14a_unwrap  */
fn main() {
    let tal = String::from("215");   
    let n = tal.parse::<usize>().unwrap();
    println!("Talet är {}", n);
}
// Talet är 215
