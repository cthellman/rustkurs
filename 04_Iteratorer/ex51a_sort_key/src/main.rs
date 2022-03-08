/*  ex51a_sort_key  */
fn slutsiffra(n: &usize) -> usize {
    *n % 10
}

fn main() {
    let mut lista = [12_usize, 19, 33, 27];
    lista.sort_by_key(slutsiffra);
    println!("{:?}", lista);
}
// [12, 33, 27, 19]


