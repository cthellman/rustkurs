/*  ex22a_array  */
fn main() {
    let a: [usize; 5] = [12, 19, 33, 27, 22];   // fixed size, mut makes the values changeable
    for i in 0..5 {
	print!("{} ", a[i]);
    }
}
// 12 19 33 27 22
