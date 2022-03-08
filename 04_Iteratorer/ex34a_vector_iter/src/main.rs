/*  ex34a_vector_iter  */
fn main() {
    let lista = vec![2, 3, 5, 7, 11];
    let n: usize = lista.iter().sum();
    println!("Res: {}", n);
}
// Res: 28
