/*  ex15a_fibonacci  */
fn main() {
    let mut a: usize = 0;
    let mut b = 1;
    while a < 500 {
	print!("{} ", a);
	let c = a + b;
	a = b;
	b = c;
    }
    println!();
}
// 0 1 1 2 3 5 8 13 21 34 55 89 144 233 377
