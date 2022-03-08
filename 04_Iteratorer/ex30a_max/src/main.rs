/*  ex30a_max  */
fn main() {
    let lista = vec![12, 19, 33, 27];
    if let Some(&n) = lista.iter().max() {    
	println!("Störst: {}", n);
    }
}
// Störst: 33

