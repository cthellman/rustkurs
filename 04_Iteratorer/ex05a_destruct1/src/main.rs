/*  ex05a_destruct1  */
fn main() {
    let t = ("Ken", 27);
    if let ("Ken", n) = t {
	println!("{}", n);
    }
}
// 27

