/*  ex52a_closure  */
fn main() {
    let mut lista = [12_usize, 19, 22, 33, 27, 13];
    lista.sort_by_key(|n| *n % 10);
    println!("{:?}", lista);
}
// [12, 22, 33, 13, 27, 19]

