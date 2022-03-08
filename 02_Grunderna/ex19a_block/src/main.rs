/*  ex19a_block  */
fn main() {
    let a = 2;
    let b = {
	let x = 3;
	let y = 4;
	a + x * y
    };
    println!("Resultat: {}", b);
}
// Resultat: 14
