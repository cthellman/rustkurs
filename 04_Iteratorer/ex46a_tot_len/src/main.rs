/*  ex46a_tot_len  */
fn main() {
    let lista = vec![
	"Bill".to_string(), "Steve".to_string(),
	"Linus".to_string(), "Ken".to_string(),
    ];
    let n: usize = lista.iter().map(|s| s.len()).sum();
    println!("Resultat: {}", n);
}
// Resultat: 17

