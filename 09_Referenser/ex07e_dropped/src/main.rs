/*  ex07e_dropped  */
fn max(v: &[usize]) -> &usize {
    v.iter().max().unwrap()
}

fn main() {
    let m;
    {
	let tal: Vec<usize> = vec![12, 19, 33, 27];
	m = max(&tal);
    }
    println!("Värdet är {}", *m);
}
// error[E0597]: `tal` does not live long enough


