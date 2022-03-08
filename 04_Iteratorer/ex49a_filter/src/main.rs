/*  ex49a_filter  */
fn main() {
    let s = "Bill 12\nLinus 33";
    let n = s.chars().
	filter(|&c| match c {
	'0'..='9' => true,
	_ => false,
    }).count();
    println!("{} siffror.", n);
}
// 4 siffror.

