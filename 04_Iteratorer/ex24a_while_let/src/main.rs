/*  ex24a_while_let  */
fn main() {
    let s = "Bill 12\nSteve 19\n\
	     Linus 33\nKen 27\n";
    let mut it = s.split_whitespace();
    while let Some(namn) = it.next() {
	if let Some(p) = it.next() {
	    print!("{} fick {}. ", namn, p);
	}
    }
}
// Bill fick 12. Steve fick 19. Linus fick 33. Ken fick 27.

