/*  ex32a_str_iters  */
fn main() {
    let namn = "Göran";
    for c in namn.chars() {
	print!("{} ", c);
    }
    for c in namn.bytes() {
	print!("{} ", c);
    }
}
// G ö r a n 71 195 182 114 97 110


