/*  ex08a_destruct4  */
fn f(&tal: &usize) {
    println!("{}", tal);
}

fn main() {
    let n: usize = 27;
    f(&n);
}
// 27

