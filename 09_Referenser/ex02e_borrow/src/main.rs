/*  ex02e_borrow  */
fn main() {
    let mut lista = vec![2, 3, 5, 7, 11];
    for &x in &lista {
	if x == 5 { lista.remove(2); }
	       //   ^^^^^^^^^^^^^^^ mutable borrow occurs here
    }
    println!("Lista: {:?}", lista);
}
// error[E0502]: cannot borrow `lista` as mutable because it is also borrowed as immutable

