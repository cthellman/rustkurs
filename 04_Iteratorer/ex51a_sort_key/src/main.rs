/*  ex51a_sort_key  */
// fn slutsiffra(n: &usize) -> usize {
//     *n % 10
// }

fn main() {
    let mut lista = [12_usize, 19, 33, 27];
    // lista.sort_by_key(slutsiffra);   // function solution
    lista.sort_by_key(|f|*f%10); // closure solution       
    println!("{:?}", lista);
}
// [12, 33, 27, 19]


