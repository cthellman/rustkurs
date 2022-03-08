/*  ex28a_iter_mut  */
fn main() {
    let mut primtal = [2, 3, 5, 7, 11];
    let mut it = primtal.iter_mut();
    while let Some(tal_ref) = it.next() {
	*tal_ref += 1;
    }
    println!("{:?} ", primtal);
}
// [3, 4, 6, 8, 12] 
