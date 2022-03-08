/*  ex22a_next  */
fn main() {
    let s = "Ett två tre fyra.";
    let mut it = s.split_whitespace();
    loop {
	if let Some(ord) = it.next() {
	    println!("Ord: {}", ord);
	} else {
	    break;
	}
    }
}
// Ord: Ett
// Ord: två
// Ord: tre
// Ord: fyra.
