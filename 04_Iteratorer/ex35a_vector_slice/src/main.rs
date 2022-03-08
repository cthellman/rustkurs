/*  ex35a_vector_slice  */
fn main() {
    let lista = vec![2, 3, 5, 7, 11, 13];
    println!("Summa: {}", summa(&lista[1..5]));
    println!("Summa: {}", summa(&lista[2..]));
}

fn summa(sekvens: &[usize]) -> usize {
    sekvens.iter().sum()
}
// Summa: 26
// Summa: 36


