/*  ex14a_index  */
fn main() {
    let mut lista = [2, 3, 5, 7, 11];
    for i in 0..lista.len() {
	lista[i] = lista[i] + 1;
    }
    println!("Array: {:?}", lista);
}
// Array: [3, 4, 6, 8, 12]


