/*  ex23a_mut_array  */
fn main() {
    let mut a = [12, 19, 33, 27, 22];
    for i in 0..5 {
	a[i] = a[i] - 10;
	print!("{} ", a[i]);
    }
}
// 2 9 23 17 12 
