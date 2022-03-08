/*  ex11a_get  */
fn main() {
    let lista: [usize; 5] = [2, 3, 5, 7, 11];
    let i = rand::random::<usize>() % 6;
    if let Some(n) = lista.get(i) {
	println!("{}", n);
    } else {
	println!("Index {} är för stort", i);
    }
}
// 5

