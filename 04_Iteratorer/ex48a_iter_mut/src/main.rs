/*  ex48a_iter_mut  */
fn main() {
    let mut lista = vec![
	"Bill".to_string(), "Steve".to_string(),
	"Linus".to_string(), "Ken".to_string(),
    ];
    if let Some(namn) = lista.iter_mut()
	.find(|s| s.len() == 3) {
	namn.push_str("ny");
	println!("Namn: {}", namn);
    }
}
// Namn: Kenny

