/*  ex44a_for_ref  */
fn main() {
    let lista = vec![
	"Bill".to_string(), "Steve".to_string(),
	"Linus".to_string(), "Ken".to_string(),
    ];
    for namn_ref in &lista {
	print!("{} ", *namn_ref);
    }
    println!("{:?}", lista);
}
// Bill Steve Linus Ken ["Bill", "Steve", "Linus", "Ken"]

