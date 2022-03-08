/*  ex07a_destruct3  */
fn main() {
    let t = ("Ken", &27);
    if let ("Ken", &n) = t {
	println!("{}", n);
    }
}
// 27

