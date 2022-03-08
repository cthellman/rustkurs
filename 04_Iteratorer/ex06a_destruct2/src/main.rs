/*  ex06a_destruct2  */
fn main() {
    let t = ("Ken", &27);
    if let ("Ken", n) = t {
	println!("{}", *n);
    }
}
// 27

