/*  ex45a_for  */
fn main() {
    let lista = vec![
	"Bill".to_string(), "Steve".to_string(),
	"Linus".to_string(), "Ken".to_string(),
    ];
    for namn in &lista {
	print!("{} ", namn);
    }
}
// Bill Steve Linus Ken

