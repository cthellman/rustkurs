/*  ex47a_find  */
fn main() {
    let lista = vec![
	"Bill".to_string(), "Steve".to_string(),
	"Linus".to_string(), "Ken".to_string(),
    ];
    if let Some(namn_ref) = lista.iter()
	.find(|s| (**s).len() == 3) {   // iter add a reference and 
                                            // find adds a reference, which is
                                            // why the double star in front of the s
	println!("Namn: {}", *namn_ref);
    }
}
// Namn: Ken

