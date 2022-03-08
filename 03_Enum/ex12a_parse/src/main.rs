/*  ex12a_parse  */
fn main() {
    let res: Result<usize, _> = "215".parse();
    match res {
	Ok(n) => println!("Talet är {}", n),
	Err(fel) => println!("Ej tal: {}.", fel),
    }
}
// Talet är 215
