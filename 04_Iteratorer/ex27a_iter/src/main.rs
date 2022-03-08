/*  ex27a_iter  */
fn main() {
    let primtal = [2, 3, 5, 7, 11];
    let mut it = primtal.iter();
    while let Some(tal_ref) = it.next() {
	print!("{} ", *tal_ref);
    }
}
// 2 3 5 7 11 

