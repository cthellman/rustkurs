/*  ex39a_skip  */
fn main() {
    let lista = vec![2, 3, 5, 7, 11];
    let n: usize = lista.iter().skip(2).sum();
    println!("Summa: {}", n);
}
// Summa: 23


