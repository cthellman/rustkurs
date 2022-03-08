/*  ex26a_for_howto  */
fn main() {
    let primtal = [2, 3, 5, 7, 11];
    // Så här gör en for-loop:
    let mut it = primtal.into_iter();
    while let Some(tal) = it.next() {
	print!("{} ", tal);
    }
}
// 2 3 5 7 11 

