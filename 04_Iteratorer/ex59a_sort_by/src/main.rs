/*  ex59a_sort_by  */
fn main() {
    let mut lista = [12_usize, 19, 22, 33, 27, 13];
    lista.sort_by(|a, b| a.cmp(b) );
    println!("{:?}", lista);
}
// [12, 13, 19, 22, 27, 33]

