/*  ex13a_kopia  */
fn main() {
    let primtal = [2, 3, 5, 7, 11];
    let mut kopia = primtal;
    if primtal == kopia {
	println!("Lika!");
    }
    kopia[2] = 13;
    if primtal == kopia {
	println!("Lika???");
    }
}
// Lika!


