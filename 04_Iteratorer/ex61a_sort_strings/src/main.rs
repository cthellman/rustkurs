/*  ex61a_sort_strings  */
fn main() {
    let mut lista = vec![
	"Bill".to_string(), "Steve".to_string(),
	"Linus".to_string(), "Ken".to_string(),
    ];
    lista.sort_by(|a, b|
	b.len().cmp(&b.len()).then(a.cmp(b))
    );
    println!("Resultat: {:?}", lista);
}
// Resultat: ["Bill", "Ken", "Linus", "Steve"]
