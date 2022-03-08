/*  ex38a_map  */
fn main() {
    let lista = vec![2_usize, 3, 5, 7, 11];
    let v: Vec<_>;
    v = lista.iter().map(|n| *n+3).collect();
    println!("Lista: {:?}", v);
}
// Lista: [5, 6, 8, 10, 14]

