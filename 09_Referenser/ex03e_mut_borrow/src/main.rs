/*  ex03e_mut_borrow  */
fn main() {
    let mut lista = vec![2, 3, 5, 7, 11];
    for x in &mut lista {
	if *x == 5 { lista.remove(2); }
	        //   ^^^^^^^^^^^^^^^ second mutable borrow occurs here
    }
    println!("Lista: {:?}", lista);
}
// error[E0499]: cannot borrow `lista` as mutable more than once at a time


