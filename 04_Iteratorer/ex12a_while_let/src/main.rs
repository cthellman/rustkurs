/*  ex12a_while_let  */
fn main() {
    let lista: [usize; 5] = [2, 3, 5, 7, 11];
    let f = rand::random::<usize>;

    while let Some(n) = lista.get(f() % 6) {
	println!("{}", n);
    }
}
// 5
// 5
// 11


