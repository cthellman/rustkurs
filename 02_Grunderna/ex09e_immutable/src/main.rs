/*  ex09e_immutable  */
fn main() {
    let a: usize = 5;
    a = a + 10;
    println!("Resultat: {}", a);
}
// error[E0384]: cannot assign twice to immutable variable


