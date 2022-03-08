/*  ex16a_read_line  */
fn main() {
    let mut namn = String::new();
    println!("Vad heter du?");
    std::io::stdin().read_line(&mut namn)
	.expect("Kunde inte läsa från stdin");
    println!("Hej, {}!", namn);
}
// Vad heter du?
// Göran
// Hej, Göran
// !
