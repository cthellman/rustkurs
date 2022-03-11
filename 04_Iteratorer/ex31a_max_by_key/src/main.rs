/*  ex31a_max_by_key  */
fn main() {
    let lista = vec![("Bill", 12),
	("Steve", 19), ("Linus", 33),
	("Ken", 27)];
    if let Some(tref) = lista.iter()
	.max_by_key(|t| t.1) {	// The function is called closure and can be written:
										// fn ps(t:(&str, usize)) -> usize {t.1}
	println!("Vinnare: {:?}", tref);
    }
}
// Vinnare: ("Linus", 33)

