/*  ex10a_mutable  */
fn main() {
    let mut a: usize = 5;
    a = a + 10;
    println!("Resultat: {a}");
}
// Resultat: 15

/*

Detta kräver Rust 1.58 eller
nyare eftersom variabeln a är
inbäddad i formatsträngen.

*/
