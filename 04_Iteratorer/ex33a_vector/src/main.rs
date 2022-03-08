/*  ex33a_vector  */
fn main() {
    let mut lista: Vec<usize> = Vec::new();
    lista.push(2); lista.push(3);
    lista.push(5); lista.push(7);
    println!("Lista: {:?}", lista);
    if let Some(n) = lista.pop() {
	println!("Tog bort {}", n);
    }
}
// Lista: [2, 3, 5, 7]
// Tog bort 7

