/*  ex01e_moved  */
fn main() {
    let mut lista = vec![2, 3, 5, 7, 11];
    for x in lista {
	if x == 5 { lista.remove(2); }
               //   ^^^^^^^^^^^^^^^ value borrowed here after move
    }
    println!("Lista: {:?}", lista);
}
// error[E0382]: borrow of moved value: `lista`

