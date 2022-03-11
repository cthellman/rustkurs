/*  ex08a_sort_f64 */
fn main() {
    let mut lista = [12.3, 18.9, 33.4, 27.0_f64];
    if lista.iter().find(|&x| x.is_nan()) == None {
	lista.sort_by(
	    |x, y| x.partial_cmp(y).unwrap() // unwrap Some value here
	);
	println!("{:?}", lista);
    }
}
// [12.3, 18.9, 27.0, 33.4]





