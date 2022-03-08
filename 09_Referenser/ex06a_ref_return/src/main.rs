/*  ex06a_ref_return  */
fn max(v: &[usize]) -> &usize {
    v.iter().max().unwrap()
}

fn main() {
    let tal: Vec<usize> = vec![12, 19, 33, 27];
    let m = max(&tal);
    println!("Värdet är {}", *m);
}
// Värdet är 33



