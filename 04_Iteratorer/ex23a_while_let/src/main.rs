/*  ex23a_while_let  */
fn main() {
    let s = "Ett två tre fyra.";
    let mut it = s.split_whitespace();
    while let Some(ord) = it.next() {
	println!("Ord: {}", ord);
    }
}
// Ord: Ett
// Ord: två
// Ord: tre
// Ord: fyra.
