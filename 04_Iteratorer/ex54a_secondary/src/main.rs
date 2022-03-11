/*  ex54a_secondary  */
fn main() {
    let mut lista = [12_usize, 19, 22, 33, 27, 13];
    lista.sort_by_key(|n| (*n % 10, *n) );  // First sort on the single digit then on the 
    println!("{:?}", lista);                       // then on the number
}
// [12, 22, 13, 33, 27, 19]


